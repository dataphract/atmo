use std::str::FromStr;

use atmo_core::{Did, Handle};
use serde_json::json;

use crate::com::atproto::server::create_session;

#[test]
fn create_session_input() {
    let input = create_session::Input {
        identifier: "me@example.com".into(),
        password: "hunter2".into(),
        auth_factor_token: None,
        allow_takendown: None,
    };

    let expected = json!({
        "identifier": "me@example.com",
        "password": "hunter2",
    });

    let serialized = serde_json::to_value(&input).unwrap();
    assert_eq!(&expected, &serialized);
    let deserialized: create_session::Input = serde_json::from_value(serialized).unwrap();
    assert_eq!(&input, &deserialized);
}

#[test]
fn create_session_output() {
    let output = create_session::Output {
        access_jwt: "not a real jwt".into(),
        active: Some(true),
        did: Did::from_str("did:web:notarealuser.example.com").unwrap(),
        did_doc: None,
        email: Some("notarealuser@example.com".into()),
        email_auth_factor: None,
        email_confirmed: Some(true),
        handle: Handle::from_str("notarealuser.example.com").unwrap(),
        refresh_jwt: "also not a real jwt".into(),
        status: None,
    };

    let expected = json!({
        "accessJwt": "not a real jwt",
        "active": true,
        "did": "did:web:notarealuser.example.com",
        "email": "notarealuser@example.com",
        "emailConfirmed": true,
        "handle": "notarealuser.example.com",
        "refreshJwt": "also not a real jwt",
    });

    let serialized = serde_json::to_value(&output).unwrap();
    assert_eq!(&expected, &serialized);
    let deserialized: create_session::Output = serde_json::from_value(serialized).unwrap();
    assert_eq!(&output, &deserialized);
}
