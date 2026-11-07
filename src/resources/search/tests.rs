#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn facet_path_repeats_the_bracketed_key() {
    let facets = vec!["status".to_string(), "treatmentPlan".to_string()];
    let path = facet_path(QueryPath::new("/risks-search"), &facets);
    assert_eq!(path, "/risks-search?facets%5B%5D=status&facets%5B%5D=treatmentPlan");
}

#[test]
fn facet_path_keeps_existing_query() {
    let mut q = QueryPath::new("/risks-search");
    q.push("q", "x");
    let path = facet_path(q, &["type".to_string()]);
    assert_eq!(path, "/risks-search?q=x&facets%5B%5D=type");
}

#[test]
fn facet_path_without_facets_is_unchanged() {
    assert_eq!(facet_path(QueryPath::new("/risks-search"), &[]), "/risks-search");
}
