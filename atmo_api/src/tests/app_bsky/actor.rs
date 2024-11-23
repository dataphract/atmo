use std::str::FromStr;

use atmo_core::DateTimeString;

use crate::app::bsky::actor::{
    defs::{PersonalDetailsPref, Preferences},
    get_preferences::Output,
};

#[test]
fn get_preferences_output() {
    let birthday_str = "1979-05-25T14:30:00.123456Z";
    let birth_date = DateTimeString::from_str(birthday_str).unwrap();

    let output = Output {
        preferences: vec![Preferences::PersonalDetailsPref(PersonalDetailsPref {
            birth_date: Some(birth_date),
        })],
    };

    let serialized = serde_json::to_value(&output).unwrap();

    let expected = serde_json::json!({
        "preferences": [
            {
                "$type": "app.bsky.actor.defs#personalDetailsPref",
                "birthDate": birthday_str,
            }
        ]
    });

    assert_eq!(serialized, expected);

    let deserialized: Output = serde_json::from_value(serialized).unwrap();

    assert_eq!(output, deserialized);
}
