use rust_cli::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = parse_config(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing config: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Error running: {}", e);
        std::process::exit(1);
    }
}
