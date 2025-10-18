/// CLI argument parsing and help display
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

pub struct Args {
    pub show_help: bool,
    pub show_version: bool,
}

impl Args {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        
        Args {
            show_help: args.iter().any(|arg| arg == "--help" || arg == "-h"),
            show_version: args.iter().any(|arg| arg == "--version" || arg == "-v"),
        }
    }
}

pub fn print_version() {
    println!("{} v{}", NAME, VERSION);
}

pub fn print_help() {
    println!("{} v{}", NAME, VERSION);
    println!("\nA simple HTTP file server for local development");
    println!("\nUSAGE:");
    println!("    {} [OPTIONS]", NAME);
    println!("\nOPTIONS:");
    println!("    -h, --help       Print help information");
    println!("    -v, --version    Print version information");
    println!("\nEXAMPLES:");
    println!("    {}              Start the server in current directory", NAME);
    println!("    {} --help       Show this help message", NAME);
}
