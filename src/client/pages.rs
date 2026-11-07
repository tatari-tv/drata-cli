//! Page-number pagination for SafeBase list endpoints.
//!
//! SafeBase has no cursor. Three list ops page by number, each a little
//! differently, so one loop runs off a per-op descriptor (`PagedOp`):
//!
//! | Op | Request | Items | Stop |
//! |---|---|---|---|
//! | `GET /accounts` | `pageNumber` from 1, `pageSize` 1000 | `result[]` | `pageNumber >= pagination.totalPages` |
//! | `GET /documents` | `page` from 0, `limit` 100 | `result.documents[]` | collected `>= result.count` |
//! | `GET /kb/search` | `pageNumber` from 1, `pageSize` 1000 | `result.items[]` | `pageNumber >= pagination.totalPages` |
//!
//! Page sizes are the maximums the Tatari tenant honored (design doc, Phase 0
//! Observed). `/accounts` always sends `pageNumber`: without it `totalPages`
//! comes back null.
//!
//! A misbehaving server is an error, never an empty success or a silently
//! short result (`PageError`). Each page is checked before any of its items
//! reach the sink, so a failure under NDJSON streaming leaves the earlier
//! pages' lines written and none of the bad page's. Drata's cursor loop
//! (`DrataClient::drain_pages`) is separate and unchanged.

use super::{DrataClient, MAX_PAGES};
use crate::backend::Backend;
use eyre::{Context, Result, bail};
use serde_json::Value;
use std::io::Write;
use thiserror::Error;
use tracing::{debug, instrument, trace};

/// Where a paged op reads its stop condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    /// Stop once the page just fetched (1-based) reaches the integer at this
    /// JSON pointer.
    TotalPages(&'static str),
    /// Stop once the items collected reach the integer at this JSON pointer.
    Count(&'static str),
}

/// Everything that differs between SafeBase's paged list ops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PagedOp {
    /// The op's path, without a query string.
    pub path: &'static str,
    /// Query parameter carrying the page number.
    pub page_param: &'static str,
    /// The first page's number: 1, or 0 for `/documents`.
    pub first_page: u32,
    /// Query parameter carrying the page size.
    pub size_param: &'static str,
    /// Page size sent on every request.
    pub page_size: u32,
    /// JSON pointer to the page's items array.
    pub items: &'static str,
    pub stop: Stop,
    /// Most pages one listing may take.
    pub max_pages: u32,
    /// Where `max_pages` comes from, for the error that names it.
    pub cap_source: &'static str,
}

/// `GET /accounts`. The spec bounds `pageNumber` at 500.
pub const ACCOUNTS: PagedOp = PagedOp {
    path: "/accounts",
    page_param: "pageNumber",
    first_page: 1,
    size_param: "pageSize",
    page_size: 1000,
    items: "/result",
    stop: Stop::TotalPages("/pagination/totalPages"),
    max_pages: 500,
    cap_source: "the spec's pageNumber bound",
};

/// `GET /documents`. Zero-based `page`; no pagination object, so it stops on
/// `result.count`.
pub const DOCUMENTS: PagedOp = PagedOp {
    path: "/documents",
    page_param: "page",
    first_page: 0,
    size_param: "limit",
    page_size: 100,
    items: "/result/documents",
    stop: Stop::Count("/result/count"),
    max_pages: MAX_PAGES,
    cap_source: "the client's hard page cap",
};

/// `GET /kb/search`. `result.count` is this page's item count, not a total,
/// so it stops on `pagination.totalPages`.
pub const KB_SEARCH: PagedOp = PagedOp {
    path: "/kb/search",
    page_param: "pageNumber",
    first_page: 1,
    size_param: "pageSize",
    page_size: 1000,
    items: "/result/items",
    stop: Stop::TotalPages("/pagination/totalPages"),
    max_pages: MAX_PAGES,
    cap_source: "the client's hard page cap",
};

/// A page that breaks its op's paging contract. Every variant names the
/// request (`path` and `param=value`) that produced the page.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PageError {
    #[error("pagination on {path} broke at {page}: `{field}` is missing or not {expected}")]
    BadField {
        path: String,
        page: String,
        field: &'static str,
        expected: &'static str,
    },
    #[error(
        "pagination on {path} broke at {page}: empty page before the stop condition \
         ({collected} items so far, {stop})"
    )]
    EmptyPage {
        path: String,
        page: String,
        collected: u64,
        stop: String,
    },
    #[error(
        "pagination on {path} broke at {page}: repeated page, its items equal the previous page's \
         (the server is ignoring `{param}`)"
    )]
    RepeatedPage {
        path: String,
        page: String,
        param: &'static str,
    },
    #[error("pagination on {path} broke at {page}: collected {collected} items, more than count {count}")]
    CountExceeded {
        path: String,
        page: String,
        collected: u64,
        count: u64,
    },
    #[error("pagination on {path} broke at {page}: page past the last page (totalPages {total}) carried {items} items")]
    PastLastPage {
        path: String,
        page: String,
        total: u64,
        items: usize,
    },
    #[error(
        "pagination on {path} hit the {max}-page cap ({source_name}) at {page}: \
         {reason}; refusing to return a truncated result"
    )]
    PageCap {
        path: String,
        page: String,
        max: u32,
        source_name: &'static str,
        reason: String,
    },
}

/// One validated page: its items and whether the listing is done.
struct Page {
    items: Vec<Value>,
    done: bool,
}

impl PagedOp {
    /// `param=value` for the request at zero-based `index`.
    fn page_label(&self, index: u32) -> String {
        format!("{}={}", self.page_param, self.first_page + index)
    }

    /// The URL path for the page at zero-based `index`.
    fn page_path(&self, path: &str, index: u32) -> String {
        let sep = if path.contains('?') { '&' } else { '?' };
        format!(
            "{}{}{}={}&{}={}",
            path,
            sep,
            self.page_param,
            self.first_page + index,
            self.size_param,
            self.page_size
        )
    }

    /// Check one page against the op's contract. `index` is zero-based,
    /// `collected` counts items from earlier pages, `prev` is the previous
    /// page's items. Pure, so every rule is unit-tested without a server.
    fn check(&self, path: &str, index: u32, collected: u64, prev: Option<&[Value]>, resp: &Value) -> Result<Page> {
        let page = self.page_label(index);
        let bad_field = |field: &'static str, expected: &'static str| PageError::BadField {
            path: path.to_string(),
            page: page.clone(),
            field,
            expected,
        };

        let items = resp
            .pointer(self.items)
            .and_then(Value::as_array)
            .ok_or_else(|| bad_field(self.items, "an array"))?
            .clone();
        let (stop_field, stop_value) = match self.stop {
            Stop::TotalPages(field) | Stop::Count(field) => (field, resp.pointer(field).and_then(Value::as_u64)),
        };
        let stop_value = stop_value.ok_or_else(|| bad_field(stop_field, "a non-negative integer"))?;
        let fetched = u64::from(index) + 1;
        let total = collected + items.len() as u64;

        if let Stop::TotalPages(_) = self.stop
            && stop_value > u64::from(self.max_pages)
        {
            return Err(PageError::PageCap {
                path: path.to_string(),
                page,
                max: self.max_pages,
                source_name: self.cap_source,
                reason: format!("the server reports {} pages", stop_value),
            }
            .into());
        }

        if items.is_empty() {
            if index == 0 && stop_value == 0 {
                return Ok(Page { items, done: true });
            }
            let stop = match self.stop {
                Stop::TotalPages(_) => format!("totalPages {}", stop_value),
                Stop::Count(_) => format!("count {}", stop_value),
            };
            return Err(PageError::EmptyPage {
                path: path.to_string(),
                page,
                collected,
                stop,
            }
            .into());
        }

        if prev == Some(items.as_slice()) {
            return Err(PageError::RepeatedPage {
                path: path.to_string(),
                page,
                param: self.page_param,
            }
            .into());
        }

        let done = match self.stop {
            Stop::TotalPages(_) => {
                if fetched > stop_value {
                    return Err(PageError::PastLastPage {
                        path: path.to_string(),
                        page,
                        total: stop_value,
                        items: items.len(),
                    }
                    .into());
                }
                fetched >= stop_value
            }
            Stop::Count(_) => {
                if total > stop_value {
                    return Err(PageError::CountExceeded {
                        path: path.to_string(),
                        page,
                        collected: total,
                        count: stop_value,
                    }
                    .into());
                }
                total >= stop_value
            }
        };
        Ok(Page { items, done })
    }
}

impl DrataClient {
    /// Fetch every page of a SafeBase paged list op and return all items.
    /// `path` is `op.path` plus any filter query string. The list commands'
    /// default (buffered) mode.
    #[instrument(skip(self))]
    pub async fn get_all_pages(&self, op: &PagedOp, path: &str) -> Result<Vec<Value>> {
        let mut all = Vec::new();
        let (items, pages) = self
            .drain_numbered_pages(op, path, |item| {
                all.push(item.clone());
                Ok(())
            })
            .await?;
        debug!(total = items, pages, "page-number pagination complete");
        Ok(all)
    }

    /// Stream every page of a SafeBase paged list op to `writer` as NDJSON,
    /// one item per line, returning the item count. The `--all` mode. On a
    /// mid-stream failure the lines already written stay written, and the
    /// error names the page.
    #[instrument(skip(self, writer))]
    pub async fn stream_all_pages<W: Write>(&self, op: &PagedOp, path: &str, writer: &mut W) -> Result<u64> {
        let (items, pages) = self
            .drain_numbered_pages(op, path, |item| {
                let line = serde_json::to_string(item).context("Failed to serialize item as NDJSON")?;
                writeln!(writer, "{}", line).context("Failed to write NDJSON line")?;
                Ok(())
            })
            .await?;
        debug!(total_items = items, pages, "stream_all_pages complete");
        Ok(items)
    }

    /// The one page-number loop behind `get_all_pages` and
    /// `stream_all_pages`. Checks each page with `PagedOp::check` before its
    /// items reach `sink`, paces requests like the cursor loop, and returns
    /// (items, pages).
    async fn drain_numbered_pages<F>(&self, op: &PagedOp, path: &str, mut sink: F) -> Result<(u64, u32)>
    where
        F: FnMut(&Value) -> Result<()>,
    {
        if self.backend != Backend::Safebase {
            bail!(
                "page-number pagination is SafeBase only; this client targets {}",
                self.backend
            );
        }
        let route = path.split('?').next().unwrap_or(path);
        if route != op.path {
            bail!("page-number pagination for {} was handed the path {}", op.path, path);
        }

        let mut collected: u64 = 0;
        let mut prev: Option<Vec<Value>> = None;
        let mut index: u32 = 0;
        let mut last_page: Option<tokio::time::Instant> = None;

        loop {
            if index >= op.max_pages {
                return Err(PageError::PageCap {
                    path: path.to_string(),
                    page: op.page_label(index),
                    max: op.max_pages,
                    source_name: op.cap_source,
                    reason: format!("{} items collected and the stop condition is not met", collected),
                }
                .into());
            }

            if let Some(at) = last_page {
                let wait = self.page_interval.saturating_sub(at.elapsed());
                if !wait.is_zero() {
                    trace!(wait_ms = wait.as_millis() as u64, "pacing page request");
                    tokio::time::sleep(wait).await;
                }
            }
            last_page = Some(tokio::time::Instant::now());

            let resp = self
                .get(&op.page_path(path, index))
                .await
                .wrap_err_with(|| format!("{} failed at {} after {} items", path, op.page_label(index), collected))?;
            let page = op.check(path, index, collected, prev.as_deref(), &resp)?;
            trace!(page = %op.page_label(index), items = page.items.len(), done = page.done, "draining page");

            for item in &page.items {
                sink(item)?;
            }
            collected += page.items.len() as u64;
            index += 1;
            if page.done {
                break;
            }
            prev = Some(page.items);
        }

        Ok((collected, index))
    }
}

#[cfg(test)]
mod tests;
