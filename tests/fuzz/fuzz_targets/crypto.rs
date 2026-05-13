#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Fuzz crypto operations with arbitrary input
    if data.len() >= 32 {
        let key: [u8; 32] = data[0..32].try_into().unwrap_or([0u8; 32]);
        let nonce = [0u8; 12];
        let plaintext = &data[32..];
        let aad = b"fuzz";

        let _ = qrd_core::crypto::Crypto::encrypt(&key, &nonce, plaintext, aad);
    }
});
