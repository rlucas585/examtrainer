use examtrainer::test_runner::development_func;

fn main() {
    if let Err(e) = development_func() {
        println!("{}", e);
    }
}
