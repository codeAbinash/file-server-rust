/// Utility functions for file size formatting
use std::path::{Path, PathBuf};

const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;

pub fn format_size(size: u64) -> String {

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

pub fn decode_path(base: &Path, url_path: &str) -> PathBuf {
    let clean_path = url_path.trim_start_matches('/');
    let decoded = urlencoding::decode(clean_path).unwrap_or_default();
    base.join(decoded.as_ref())
}
