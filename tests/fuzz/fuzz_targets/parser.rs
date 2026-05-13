#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Fuzz the parser with arbitrary binary input
    // Ensure no panics occur on any input
    let _ = qrd_core::encoding::Decoder::decode_safe::<String>(data);
});
