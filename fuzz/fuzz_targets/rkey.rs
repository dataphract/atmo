#![no_main]

use atmo_core::RecordKey;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = RecordKey::try_from(data);
});
