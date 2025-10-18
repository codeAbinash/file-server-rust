/// HTML generation for directory listings
use std::fs;
use std::path::Path;
use crate::utils::format_size;

pub fn generate_directory_listing(dir: &Path, url_path: &str) -> Result<String, String> {
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
