use serde::de::DeserializeOwned;

// Extract "data: ..." lines from SSE bytes (excluding comments and [DONE])
pub fn extract_data_lines_from_bytes(bytes: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    for line in bytes.split(|b| *b == b'\n') {
        if line.is_empty() { continue; }
        let mut l = line;
        if let Some(last) = l.last() { if *last == b'\r' { l = &l[..l.len()-1]; } }
        if l.is_empty() || l[0] == b':' { continue; }
        if let Some(rest) = l.strip_prefix(b"data: ") {
            if rest == b"[DONE]" { continue; }
            out.push(String::from_utf8(rest.to_vec()).unwrap_or_default());
        }
    }
    out
}

// Parse JSON values from SSE bytes. Invalid JSON lines are skipped.
pub fn extract_json_values_from_bytes(bytes: &[u8]) -> Vec<serde_json::Value> {
    extract_data_lines_from_bytes(bytes)
        .into_iter()
        .filter_map(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .collect()
}

// Deserialize typed items from SSE bytes. Invalid lines are skipped.
pub fn extract_typed_from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Vec<T> {
    extract_data_lines_from_bytes(bytes)
        .into_iter()
        .filter_map(|s| serde_json::from_str::<T>(&s).ok())
        .collect()
}

// String variants
pub fn extract_data_lines_from_str(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(|l| {
            let l = l.trim_end_matches('\r');
            if l.is_empty() || l.starts_with(':') { return None; }
            l.strip_prefix("data: ")
        })
        .filter(|rest| *rest != "[DONE]")
        .collect()
}

pub fn extract_json_values_from_str(text: &str) -> Vec<serde_json::Value> {
    extract_data_lines_from_str(text)
        .into_iter()
        .filter_map(|s| serde_json::from_str::<serde_json::Value>(s).ok())
        .collect()
}

pub fn extract_typed_from_str<T: DeserializeOwned>(text: &str) -> Vec<T> {
    extract_data_lines_from_str(text)
        .into_iter()
        .filter_map(|s| serde_json::from_str::<T>(s).ok())
        .collect()
}

// Result variants (fail on first invalid JSON)
pub fn try_extract_json_values_from_str(text: &str) -> Result<Vec<serde_json::Value>, serde_json::Error> {
    let mut out = Vec::new();
    for s in extract_data_lines_from_str(text) { out.push(serde_json::from_str::<serde_json::Value>(s)?); }
    Ok(out)
}

pub fn try_extract_typed_from_str<T: DeserializeOwned>(text: &str) -> Result<Vec<T>, serde_json::Error> {
    let mut out = Vec::new();
    for s in extract_data_lines_from_str(text) { out.push(serde_json::from_str::<T>(s)?); }
    Ok(out)
}
