#![no_main]

use atmo_core::Tid;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = Tid::try_from(data);
});
