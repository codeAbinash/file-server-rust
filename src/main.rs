mod cli;
mod content_type;
mod html;
mod server;
mod utils;

use std::env;
use tiny_http::{Response, Server};

static HOST: &str = "127.28.29.30:3132";

fn main() {
    // Parse command line arguments
    let args = cli::Args::parse();
    
    if args.show_version {
        cli::print_version();
        return;
    }
    
    if args.show_help {
        cli::print_help();
        return;
    }

    // Start the server
    let server = Server::http(HOST).unwrap();
    let current_dir = env::current_dir().unwrap();

    println!("🚀 File Server Started!");
    println!("📁 Serving directory: {}", current_dir.display());
    println!("🌐 Server running at: http://{}", HOST);
    println!("Press Ctrl+C to stop\n");

    for request in server.incoming_requests() {
        let url_path = request.url();
        println!("📥 {} {}", request.method(), url_path);

        let file_path = utils::decode_path(&current_dir, url_path);

        match server::serve_path(&file_path, url_path) {
            Ok(response) => {
                let _ = request.respond(response);
            }
            Err(e) => {
                println!("   ❌ Error: {}", e);
                let response = Response::from_string(format!("Error: {}", e))
                    .with_status_code(404);
                let _ = request.respond(response);
            }
        }
    }
}
