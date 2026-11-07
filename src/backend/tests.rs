#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn drata_base_url_resolves_each_region() {
    let d = Backend::Drata;
    assert_eq!(d.base_url(Some("us")).unwrap(), DRATA_US_BASE_URL);
    assert_eq!(d.base_url(Some("eu")).unwrap(), DRATA_EU_BASE_URL);
    assert_eq!(d.base_url(Some("EU")).unwrap(), DRATA_EU_BASE_URL);
    assert_eq!(d.base_url(Some("apac")).unwrap(), DRATA_APAC_BASE_URL);
    assert_eq!(d.base_url(None).unwrap(), DRATA_US_BASE_URL);
    // Unknown region returns an error.
    assert!(d.base_url(Some("mars")).is_err());
}

#[test]
fn safebase_base_url_is_fixed() {
    assert_eq!(Backend::Safebase.base_url(None).unwrap(), SAFEBASE_BASE_URL);
}

#[test]
fn safebase_base_url_rejects_a_region() {
    let err = Backend::Safebase.base_url(Some("us")).unwrap_err().to_string();
    assert!(err.contains("Drata only"), "{err}");
}

#[test]
fn drata_auth_header_is_bearer() {
    let (name, value) = Backend::Drata.auth_header("k1").unwrap();
    assert_eq!(name, AUTHORIZATION);
    assert_eq!(value.to_str().unwrap(), "Bearer k1");
    assert!(value.is_sensitive());
}

#[test]
fn safebase_auth_header_is_sb_key() {
    let (name, value) = Backend::Safebase.auth_header("k2").unwrap();
    assert_eq!(name.as_str(), SAFEBASE_KEY_HEADER);
    assert_eq!(value.to_str().unwrap(), "k2");
    assert!(value.is_sensitive());
}

#[test]
fn auth_header_rejects_a_bad_key_without_echoing_it() {
    let err = Backend::Safebase.auth_header("bad\nkey").unwrap_err().to_string();
    assert!(err.contains("SafeBase"), "{err}");
    assert!(!err.contains("bad"), "the key must not appear in the error: {err}");
}

#[test]
fn each_backend_parses_its_own_spec() {
    let drata = Backend::Drata.spec().unwrap();
    let safebase = Backend::Safebase.spec().unwrap();
    assert_eq!(safebase["servers"][0]["url"], SAFEBASE_BASE_URL);
    assert!(drata["paths"].get("/vendors").is_some());
    assert!(safebase["paths"].get("/vendors").is_none());
    // Cached: a second call hands back the same parse.
    assert!(std::ptr::eq(safebase, Backend::Safebase.spec().unwrap()));
}

#[test]
fn display_names_the_product() {
    assert_eq!(Backend::Drata.to_string(), "Drata");
    assert_eq!(Backend::Safebase.to_string(), "SafeBase");
}
