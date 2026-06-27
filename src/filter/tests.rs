#![allow(clippy::unwrap_used)]
use super::*;

#[derive(Debug, PartialEq)]
struct Item {
    name: String,
}

impl Item {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

fn items(names: &[&str]) -> Vec<Item> {
    names.iter().map(|n| Item::new(n)).collect()
}

fn pats(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| v.to_string()).collect()
}

#[test]
fn empty_patterns_returns_all() {
    let data = items(&["alpha", "beta", "gamma"]);
    let result = filter(&data, &[], |i| &i.name);
    assert_eq!(result.len(), 3);
}

#[test]
fn exact_match_wins_over_prefix_or_contains() {
    let data = items(&["Okta", "Okta Primary", "Data Okta"]);
    let result = filter(&data, &pats(&["Okta"]), |i| &i.name);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "Okta");
}

#[test]
fn starts_with_wins_when_no_exact_match() {
    let data = items(&["Okta Primary", "Data Okta", "AWS"]);
    let result = filter(&data, &pats(&["Okta"]), |i| &i.name);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "Okta Primary");
}

#[test]
fn contains_is_used_only_when_no_exact_or_prefix_match() {
    let data = items(&["Data Okta", "Infra Okta"]);
    let result = filter(&data, &pats(&["Okta"]), |i| &i.name);
    assert_eq!(result.len(), 2);
}

#[test]
fn no_match_returns_empty() {
    let data = items(&["alpha", "beta"]);
    let result = filter(&data, &pats(&["xyz"]), |i| &i.name);
    assert!(result.is_empty());
}

#[test]
fn filter_into_matches_filter_semantics() {
    let data = items(&["Okta", "Okta Primary", "Data Okta"]);
    let result = filter_into(data, &pats(&["Okta"]), |i| &i.name);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "Okta");
}

#[test]
fn filter_into_empty_patterns_returns_all() {
    let data = items(&["alpha", "beta"]);
    let result = filter_into(data, &[], |i| &i.name);
    assert_eq!(result.len(), 2);
}

#[test]
fn case_insensitive_matching() {
    let data = items(&["Okta"]);
    let result = filter(&data, &pats(&["OKTA"]), |i| &i.name);
    assert_eq!(result.len(), 1);
    let result = filter(&data, &pats(&["oKtA"]), |i| &i.name);
    assert_eq!(result.len(), 1);
}
