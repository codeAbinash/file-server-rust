/// Server request handling and file serving
use std::fs;
use std::io::Read;
use std::path::Path;
use std::time::Instant;
use tiny_http::Response;
use crate::content_type;
use crate::html;
use crate::utils::format_size;

pub fn serve_path(path: &Path, url_path: &str) -> Result<Response<std::io::Cursor<Vec<u8>>>, String> {
    if !path.exists() {
        return Err("File not found".to_string());
    }

    if path.is_file() {
        serve_file(path)
    } else if path.is_dir() {
        serve_directory(path, url_path)
    } else {
        Err("Unknown file type".to_string())
    }
}

fn serve_file(path: &Path) -> Result<Response<std::io::Cursor<Vec<u8>>>, String> {
    let start_time = Instant::now();
    
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).map_err(|e| e.to_string())?;
    
    let elapsed = start_time.elapsed();
    let size = format_size(contents.len() as u64);
    println!("   ✓ Served {} in {:.2?}", size, elapsed);

    let content_type = content_type::guess(path);
    Ok(Response::from_data(contents).with_header(
        tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap(),
    ))
}

fn serve_directory(dir: &Path, url_path: &str) -> Result<Response<std::io::Cursor<Vec<u8>>>, String> {
    let html = html::generate_directory_listing(dir, url_path)?;
    Ok(Response::from_string(html).with_header(
        tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap(),
    ))
}
