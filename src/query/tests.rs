use super::*;
use crate::cli::VendorStatus;

#[test]
fn wire_name_is_the_spec_value() {
    assert_eq!(wire_name(&VendorStatus::OnHold), "ON_HOLD");
    assert_eq!(wire_name(&VendorStatus::None), "NONE");
}

#[test]
fn push_each_repeats_bracketed_key_per_value() {
    let mut q = QueryPath::new("/vendors");
    q.push_each("statuses[]", ["ACTIVE", "FLAGGED"]);
    assert_eq!(q.finish(), "/vendors?statuses%5B%5D=ACTIVE&statuses%5B%5D=FLAGGED");
}

#[test]
fn push_continues_an_existing_query() {
    let mut q = QueryPath::new("/vendors?size=1");
    q.push("name", "a b");
    assert_eq!(q.finish(), "/vendors?size=1&name=a%20b");
}

#[test]
fn push_opt_skips_none_and_empty_each_is_unchanged() {
    let mut q = QueryPath::new("/x");
    q.push_opt("draft", None::<bool>).push_each("ids[]", Vec::<u64>::new());
    assert_eq!(q.finish(), "/x");
}
