use serde_json::json;

use crate::com::atproto::server::create_session;

#[test]
fn create_session_input() {
    let input = create_session::Input {
        identifier: "me@example.com".into(),
        password: "hunter2".into(),
        auth_factor_token: None,
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
