use crate::config::Config;
use crate::exam::{Exam, ExamDB};
use crate::output;
use crate::question::QuestionDB;
use crate::user::User;
use crate::Error;
use crossbeam::thread;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn run(
    config: &Config,
    exam_name: &str,
    questions: &QuestionDB,
    exams: &ExamDB,
) -> Result<(), Error> {
    if let Some(exam) = exams.get_exam_by_name(exam_name) {
        run_internal(config, exam, questions)
    } else {
        println!("The exam '{}' was not found", exam_name);
        Ok(())
    }
}

fn run_internal(config: &Config, exam: &Exam, questions: &QuestionDB) -> Result<(), Error> {
    let user = Arc::new(Mutex::new(User::new()));
    let (thread_send, main_receiver) = mpsc::channel();
    let (main_send, thread_receiver) = mpsc::channel();

    let thread_user = user.clone();

    thread::scope(|s| {
        let handle = s.spawn(move |_| {
            if let Err(e) = exam_loop(config, exam, questions, thread_user, thread_receiver) {
                output::unexpected_error(e);
            }
            thread_send.send(true).unwrap();
        });

        // match main_receiver.recv_timeout(exam.duration()) {
        match main_receiver.recv_timeout(Duration::from_secs(2)) {
            Ok(_) => println!("Thread completed successfully"),
            Err(_) => output::print_timeout(),
        }

        let _ = main_send.send(true);

        handle.join().unwrap();
    })
    .map_err(|_| Error::General("Thread error".to_string()))?;

    let end_user = Arc::try_unwrap(user)
        .map_err(|_| Error::General("Thread error".to_string()))?
        .into_inner()
        .map_err(|_| Error::General("Thread error".to_string()))?;
    Ok(())
}

fn exam_loop(
    config: &Config,
    exam: &Exam,
    questions: &QuestionDB,
    user: Arc<Mutex<User>>,
    thread_receiver: Receiver<bool>,
) -> Result<(), Error> {
    loop {
        std::thread::sleep(Duration::from_millis(200));

        match thread_receiver.try_recv() {
            Ok(_) => break,
            Err(_) => (),
        }
    }
    Ok(())
}
