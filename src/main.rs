use examtrainer::config::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    if let Err(e) = examtrainer::run(config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
