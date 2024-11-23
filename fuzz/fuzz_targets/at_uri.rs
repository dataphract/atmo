#![no_main]

use std::str::FromStr;

use atmo_core::AtUri;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(s) = std::str::from_utf8(data) else {
        return;
    };

    let _ = AtUri::from_str(s);
});
