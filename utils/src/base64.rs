use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

pub fn encode(value: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(value)
}

pub fn decode(value: &str) -> Result<Vec<u8>, String> {
    URL_SAFE_NO_PAD.decode(value).map_err(|e| format!("Failed to decode base64 with message {}", e))
}

pub fn decode_unchecked(value: &str) -> Vec<u8> {
    URL_SAFE_NO_PAD.decode(value).unwrap()
}