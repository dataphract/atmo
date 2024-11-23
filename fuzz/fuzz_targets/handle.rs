#![no_main]

use atmo_core::Handle;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = Handle::try_from(data);
});
