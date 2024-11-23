#![no_main]

use libfuzzer_sys::fuzz_target;

use atmo_core::Did;

fuzz_target!(|data: &[u8]| {
    let _did = Did::try_from(data);
});
