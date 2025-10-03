#![cfg(feature = "surrealdb")]

#[test]
fn parses_well_formed_ids() {
    let r = morde_rs::surrealdb::string_to_record_id("user:alice");
    assert!(r.is_some());
    let id = r.unwrap();
    assert_eq!(id.table(), "user");
    assert_eq!(id.key().to_string(), "alice");
}

#[test]
fn rejects_empty_and_malformed() {
    assert!(morde_rs::surrealdb::string_to_record_id("").is_none());
    assert!(morde_rs::surrealdb::string_to_record_id("no-colon").is_none());
    assert!(morde_rs::surrealdb::string_to_record_id("a:b:c").is_none());
}
