use rust_cli::*;

fn main() {
    let config = parse_config(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing config: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Error running: {}", e);
        std::process::exit(1);
    }
}
