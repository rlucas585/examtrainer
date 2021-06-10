use examtrainer::config::Config;

fn main() {
    let config = Config::new_from("tst/resources/config_1.toml");
    println!("{:?}", config);
}
