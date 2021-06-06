use examtrainer::config::Config;
use examtrainer::test_runner::development_func;

fn main() {
    let config = Config::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    println!("{:?}", config);
    // if let Err(e) = development_func() {
    //     println!("{}", e);
    // }
}
