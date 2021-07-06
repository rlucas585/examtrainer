use super::YesNoAnswer::{self, Yes};
use crate::config::Config;
use crate::exam::{Exam, ExamDB};
use crate::output;
use crate::question::QuestionDB;
use crate::user::User;
use crate::utils::TimeInfo;
use crate::Error;
use crossbeam::thread;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Instant;

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
            if let Err(e) = exam_loop(
                config,
                exam,
                questions,
                thread_user,
                thread_send.clone(),
                thread_receiver,
            ) {
                output::unexpected_error(e);
            }
            // Notify main thread that Exam has been exited
            thread_send.send(true).unwrap();
        });

        // Wait for the User to press 'Enter' to start the exam, before beginning the exam
        // timer
        match main_receiver.recv() {
            Ok(_) => (),
            Err(_) => {
                let _ = main_send.send(true);
            }
        }

        match main_receiver.recv_timeout(exam.duration()) {
            Ok(_) => (),
            Err(_) => {
                // Notify exam thread that timeout has been reached
                let _ = main_send.send(true);
            }
        }

        handle.join().unwrap();
    })
    .map_err(|_| Error::General("Thread error".to_string()))?;

    let end_user = Arc::try_unwrap(user)
        .map_err(|_| Error::General("Thread error".to_string()))?
        .into_inner()
        .map_err(|_| Error::General("Thread error".to_string()))?;
    output::report_exam_result(&end_user, exam);
    Ok(())
}

fn exam_loop<'a>(
    config: &Config,
    exam: &Exam,
    questions: &'a QuestionDB,
    user: Arc<Mutex<User<'a>>>,
    thread_send: Sender<bool>,
    thread_receiver: Receiver<bool>,
) -> Result<(), Error> {
    let mut input;
    let mut user = user.lock()?; // Locked for lifetime of exam_loop

    let no_more_questions = assign_new_question(config, &mut user, exam, questions)?;
    if no_more_questions {
        return Ok(());
    }

    output::exam_intro(exam);
    super::wait_for_enter();

    thread_send
        .send(true)
        .map_err(|_| Error::General("Thread error".to_string()))?;

    let start = Instant::now();
    let end_time =
        chrono::Local::now() + chrono::Duration::seconds(exam.duration().as_secs() as i64);
    let end_instant = start + exam.duration();
    let time_info = TimeInfo::new(start, end_time, end_instant);

    output::exam_status(config, &user, exam, &time_info);
    output::you_can_start();

    loop {
        output::prompt();
        input = super::read_input()?;

        // Check to see if the exam has timed out
        match thread_receiver.try_recv() {
            Ok(_) => {
                output::print_timeout();
                break;
            }
            Err(_) => (),
        }

        match &input[..] {
            "grademe" => {
                super::grade(config, &mut user)?;
                if user.completed_exam(exam) {
                    return Ok(());
                }
                let no_more_questions = assign_new_question(config, &mut user, exam, questions)?;
                if no_more_questions {
                    return Ok(());
                }
                output::exam_status(config, &user, exam, &time_info);
            }
            "status" => output::exam_status(config, &user, exam, &time_info),
            "clear" => output::clear_screen()?,
            "help" => output::exam_help(),
            "exit" | "quit" => {
                let answer = exit()?;
                if matches!(answer, Yes) {
                    return Ok(());
                }
            }
            _ => output::unrecognised_command(&input),
        }
    }
    Ok(())
}

fn assign_new_question<'a>(
    config: &Config,
    user: &mut User<'a>,
    exam: &Exam,
    questions: &'a QuestionDB,
) -> Result<bool, Error> {
    if let Some(next_question_name) = exam.select_question(&user) {
        let question = questions
            .get_question_by_name(next_question_name)
            .ok_or(Error::General("Question not found".to_string()))?;
        let points = exam.get_points(&user);
        user.assign_question(question, points)?;
        question.create_directories(config)?;
        Ok(false)
    } else {
        output::no_more_questions();
        Ok(true)
    }
}

fn exit() -> Result<YesNoAnswer, Error> {
    println!("\nAre you sure you would like to exit the exam early (y/n)? ");
    let answer = super::ask_yes_or_no()?;
    Ok(answer)
}
