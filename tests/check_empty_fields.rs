#[test]
fn finds_missing_and_empty_fields() {
    struct Payload {
        name: Option<String>,
        email: Option<String>,
    }

    let payload = Payload { name: Some("".to_string()), email: None };
    let missing = morde_rs::check_empty_fields!(payload, [name, email]);
    assert_eq!(missing, vec!["name", "email"]);
}


#[test]
fn all_present_returns_empty_vec() {
    struct Payload {
        name: Option<String>,
        email: Option<String>,
    }

    let payload = Payload { name: Some("bob".to_string()), email: Some("bob@example.com".to_string()) };
    let missing = morde_rs::check_empty_fields!(payload, [name, email]);
    assert!(missing.is_empty());
}


#[test]
fn non_string_some_is_not_considered_missing() {
    struct Payload {
        id: Option<i32>,
        name: Option<String>,
    }

    let payload = Payload { id: Some(1), name: Some("alice".to_string()) };
    let missing = morde_rs::check_empty_fields!(payload, [id, name]);
    // id is Some(i32) (downcast fails -> treated as present), name is present
    assert!(missing.is_empty());
}
