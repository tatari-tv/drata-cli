//! Positional pattern filtering with 3-tier fallback (exact -> starts-with -> contains).
//!
//! All `list` commands accept zero or more positional patterns that filter by
//! the primary name of each item. Zero patterns means "all". With one or more
//! patterns, matching is tried in three tiers in order, and the first tier with
//! any match wins (OR semantics within a tier):
//!
//! 1. **exact** - any pattern equals the item's name (case-insensitive)
//! 2. **starts-with** - any pattern is a prefix of the item's name (case-insensitive)
//! 3. **contains** - any pattern is a substring of the item's name (case-insensitive)
//!
//! Matching lowercases both sides via `str::to_lowercase`, which follows Unicode
//! default casing rules. Lifted near-verbatim from `pagerduty-cli`.

/// Filter `items` by `patterns` using 3-tier fallback matching.
/// `name_of` extracts the name to match against for each item.
pub fn filter<'a, T, F>(items: &'a [T], patterns: &[String], name_of: F) -> Vec<&'a T>
where
    F: Fn(&T) -> &str,
{
    if patterns.is_empty() {
        return items.iter().collect();
    }

    let lowered_patterns: Vec<String> = patterns.iter().map(|p| p.to_lowercase()).collect();

    let t1: Vec<&T> = items
        .iter()
        .filter(|i| {
            let n = name_of(i).to_lowercase();
            lowered_patterns.contains(&n)
        })
        .collect();
    if !t1.is_empty() {
        return t1;
    }

    let t2: Vec<&T> = items
        .iter()
        .filter(|i| {
            let n = name_of(i).to_lowercase();
            lowered_patterns.iter().any(|p| n.starts_with(p))
        })
        .collect();
    if !t2.is_empty() {
        return t2;
    }

    items
        .iter()
        .filter(|i| {
            let n = name_of(i).to_lowercase();
            lowered_patterns.iter().any(|p| n.contains(p))
        })
        .collect()
}

/// Owned variant of `filter`: takes items by value and returns owned matches.
/// Useful when filtering `Vec<Value>` from deserialization where cloning is cheap.
pub fn filter_into<T, F>(items: Vec<T>, patterns: &[String], name_of: F) -> Vec<T>
where
    F: Fn(&T) -> &str,
{
    if patterns.is_empty() {
        return items;
    }

    let lowered_patterns: Vec<String> = patterns.iter().map(|p| p.to_lowercase()).collect();

    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    let mut t3 = Vec::new();

    for item in items {
        let name = name_of(&item).to_lowercase();
        if lowered_patterns.contains(&name) {
            t1.push(item);
        } else if lowered_patterns.iter().any(|p| name.starts_with(p)) {
            t2.push(item);
        } else if lowered_patterns.iter().any(|p| name.contains(p)) {
            t3.push(item);
        }
    }

    if !t1.is_empty() {
        return t1;
    }
    if !t2.is_empty() {
        return t2;
    }
    t3
}

#[cfg(test)]
mod tests;
