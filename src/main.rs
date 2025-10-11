use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tiny_http::{Response, Server};

static HOST: &str = "127.28.29.30:3132";

fn main() {
    let server = Server::http(HOST).unwrap();
    let current_dir = env::current_dir().unwrap();

    println!("🚀 File Server Started!");
    println!("📁 Serving directory: {}", current_dir.display());
    println!("🌐 Server running at: http://{}", HOST);
    println!("Press Ctrl+C to stop\n");

    for request in server.incoming_requests() {
        let url_path = request.url();
        println!("📥 {} {}", request.method(), url_path);

        let file_path = decode_path(&current_dir, url_path);

        match serve_path(&file_path, url_path) {
            Ok(response) => {
                let _ = request.respond(response);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
                let response = Response::from_string(format!("Error: {}", e))
                    .with_status_code(404);
                let _ = request.respond(response);
            }
        }
    }
}

fn decode_path(base: &Path, url_path: &str) -> PathBuf {
    let clean_path = url_path.trim_start_matches('/');
    let decoded = urlencoding::decode(clean_path).unwrap_or_default();
    base.join(decoded.as_ref())
}

fn serve_path(path: &Path, url_path: &str) -> Result<Response<std::io::Cursor<Vec<u8>>>, String> {
    if !path.exists() {
        return Err("File not found".to_string());
    }

    if path.is_file() {
        let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).map_err(|e| e.to_string())?;

        let content_type = guess_content_type(path);
        Ok(Response::from_data(contents).with_header(
            tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap(),
        ))
    } else if path.is_dir() {
        let html = generate_directory_listing(path, url_path)?;
        Ok(Response::from_string(html).with_header(
            tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap(),
        ))
    } else {
        Err("Unknown file type".to_string())
    }
}

fn generate_directory_listing(dir: &Path, url_path: &str) -> Result<String, String> {
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
    
    // Normalize URL path - ensure it ends with / for directories
    let normalized_url = if url_path.ends_with('/') || url_path.is_empty() {
        url_path.to_string()
    } else {
        format!("{}/", url_path)
    };
    
    let display_path = if normalized_url == "/" { 
        "/".to_string() 
    } else { 
        normalized_url.trim_end_matches('/').to_string()
    };

    let mut html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Index of {}</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 
                         'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            color: #e4e4e7;
            min-height: 100vh;
            padding: 2rem;
            line-height: 1.6;
        }}
        
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background: rgba(30, 30, 46, 0.8);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            padding: 2.5rem;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }}
        
        h1 {{
            color: #a6e3a1;
            font-size: 2rem;
            font-weight: 600;
            margin-bottom: 2rem;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }}
        
        .path {{
            color: #89b4fa;
            font-weight: 400;
        }}
        
        table {{
            width: 100%;
            border-collapse: separate;
            border-spacing: 0;
            margin-top: 1rem;
        }}
        
        th {{
            background: linear-gradient(135deg, #313244 0%, #45475a 100%);
            color: #cdd6f4;
            padding: 1rem;
            text-align: left;
            font-weight: 600;
            font-size: 0.875rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            border-bottom: 2px solid #89b4fa;
        }}
        
        th:first-child {{
            border-top-left-radius: 8px;
        }}
        
        th:last-child {{
            border-top-right-radius: 8px;
        }}
        
        td {{
            padding: 1rem;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
            transition: all 0.2s ease;
        }}
        
        tr:hover td {{
            background: rgba(137, 180, 250, 0.1);
        }}
        
        tr:last-child td:first-child {{
            border-bottom-left-radius: 8px;
        }}
        
        tr:last-child td:last-child {{
            border-bottom-right-radius: 8px;
        }}
        
        a {{
            color: #89dceb;
            text-decoration: none;
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            transition: all 0.2s ease;
            font-weight: 500;
        }}
        
        a:hover {{
            color: #b4befe;
        }}
        
        .size {{
            color: #a6adc8;
            font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
            font-size: 0.875rem;
        }}
        
        .parent-dir {{
            color: #f9e2af;
        }}
        
        @media (max-width: 768px) {{
            body {{
                padding: 1rem;
            }}
            
            .container {{
                padding: 1.5rem;
            }}
            
            h1 {{
                font-size: 1.5rem;
            }}
            
            th, td {{
                padding: 0.75rem 0.5rem;
                font-size: 0.875rem;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>📁 <span class="path">Index of {}</span></h1>
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><a href="../" class="parent-dir">📂 Parent Directory</a></td>
                    <td class="size">-</td>
                </tr>
"#,
        display_path, display_path
    );

    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    items.sort_by_key(|e| e.file_name());

    for entry in items {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let metadata = entry.metadata().ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = metadata
            .as_ref()
            .map(|m| {
                if m.is_dir() {
                    "-".to_string()
                } else {
                    format_size(m.len())
                }
            })
            .unwrap_or_else(|| "-".to_string());

        let icon = if is_dir { "📁" } else { "📄" };
        let display_name = if is_dir {
            format!("{}/", file_name)
        } else {
            file_name.clone()
        };
        
        // Build proper URL by appending to current path
        // Ensure proper path separator between directory and file
        let item_url = if url_path == "/" || url_path.is_empty() {
            format!("/{}", urlencoding::encode(&file_name))
        } else if url_path.ends_with('/') {
            format!("{}{}", url_path, urlencoding::encode(&file_name))
        } else {
            format!("{}/{}", url_path, urlencoding::encode(&file_name))
        };

        html.push_str(&format!(
            r#"                <tr>
                    <td><a href="{}">{} {}</a></td>
                    <td class="size">{}</td>
                </tr>
"#,
            item_url,
            icon,
            display_name,
            size
        ));
    }

    html.push_str(
        r#"            </tbody>
        </table>
    </div>
</body>
</html>"#,
    );

    Ok(html)
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

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

fn guess_content_type(path: &Path) -> String {
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    match extension.to_lowercase().as_str() {
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "txt" => "text/plain",
        "xml" => "application/xml",
        "zip" => "application/zip",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        _ => "application/octet-stream",
    }
    .to_string()
}
