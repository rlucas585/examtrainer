use examtrainer::config::Config;
use examtrainer::exam::ExamDB;
use examtrainer::question::QuestionDB;
use std::process::exit;

// TODO take first argument as Config file location (or accept flags potentially)
fn main() {
    let config = Config::new_from("tst/resources/config_1.toml").unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        exit(1);
    });
    let questions = QuestionDB::new(&config).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        exit(1);
    });
    let exams = ExamDB::new(&config, &questions).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        exit(1);
    });
    if let Err(e) = examtrainer::run(config, questions, exams) {
        eprintln!("Error: {}", e);
        exit(1);
    }
}
