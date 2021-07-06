pub mod help;

pub use help::*;

use crate::config::Config;
use crate::exam::Exam;
use crate::question::Question;
use crate::user::User;
use crate::utils;
use crate::Error;
use colored::*;
use std::time::{Duration, Instant};

use std::io::{self, Write};

fn print_divider_bar() {
    let (width, _) = term_size::dimensions().unwrap_or((50, 50));
    let width = if width <= 11 { 11 } else { width - 10 };
    for _ in 0..width - 1 {
        print!("=");
    }
    println!("=");
}

pub fn intro() {
    let intro = "
______  __   _   __  __ _____ ___    _   ___ _  _ ___ ___ 
| __\\ \\/ /  /_\\ |  \\/  |_   _| _ \\  /_\\ |_ _| \\| | __| _ \\
| _| >  <  / _ \\| |\\/| | | | |   / / _ \\ | || .` | _||   /
|___/_/\\_\\/_/ \\_\\_|  |_| |_| |_|_\\/_/ \\_\\___|_|\\_|___|_|_\\
";
    println!("{}", intro.green());

    println!("Welcome to Examtrainer\n");
    println!(
        "This program was initially designed as a tool to practice for exams within the 42 \
        Curriculum, but it can be used to practice any basic programming exercises.\n"
    );

    println!(
        "You are currently at the main menu, type \"help\" to see a list of possible commands\n"
    );
}

pub fn main_menu_prompt() {
    print!("{}> ", "examshell-admin".yellow());
    let _ = io::stdout().flush();
}

pub fn prompt() {
    print!("{}> ", "examshell".yellow());
    let _ = io::stdout().flush();
}

pub fn unrecognised_command(command: &str) {
    println!(
        "Unrecognised command: '{}', type '{}' for a list of possible commands",
        command,
        "help".green()
    );
}

pub fn clear_screen() -> Result<(), Error> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().map_err(|e| e.into())
}

fn print_directory_info(config: &Config, question: &Question) {
    println!(
        "The subject is located at {}",
        format!("{}/{}", config.submit_dir(), question.name()).green()
    );
    println!("You must turn in your files in a subdirectory of your submit directory");
    println!(
        "with the same name as your assignment ({})",
        question.directories().submit_directory.red()
    );
    println!(
        "Examtrainer does not require you to {}, but remember to do this in the real exam!!",
        "git push".red()
    );
}

pub fn single_question_status(config: &Config, user: &User) -> Result<(), Error> {
    if let Some(question) = user.current_question() {
        print_divider_bar();
        println!("Examshell: Single Question Mode\n");
        println!("Your question is {}", question.name().green());
        print_directory_info(config, question);
        print_divider_bar();
        Ok(())
    } else {
        Err(Error::General(
            "User was not given assignment correctly".to_string(),
        ))
    }
}

pub fn single_question_intro(question: &Question) {
    println!(
        "You are registered to begin the question: {}",
        question.name().green()
    );
}

fn duration_print(duration: &Duration) {
    let (hours, minutes, seconds) = utils::seconds_to_hours_and_minutes(duration.as_secs());

    let time = format!("{}hrs, {}mins and {}sec", hours, minutes, seconds).green();
    print!("{}", time);
}

pub fn exam_intro(exam: &Exam) {
    println!("You are registered to begin the exam: {}", exam.name());
    print!("You will have ");
    duration_print(&exam.duration());
    println!(" to complete this exam");
    println!("Time will begin once you press enter to continue");
}

pub fn print_success() {
    let success = format!(
        "{}\n{}\n{}\n",
        "===========================================",
        "=                 SUCCESS                 =",
        "===========================================",
    )
    .green();
    println!("{}", success);
    println!("You have passed the assignment\n");
}

pub fn print_failure() {
    let failure = format!(
        "{}\n{}\n{}\n",
        "===========================================",
        "=                 FAILURE                 =",
        "===========================================",
    )
    .red();
    println!("{}", failure);
    println!("You have failed the assignment\n");
}

pub fn print_config_info(config: &Config) {
    println!("{}", config);
}

pub fn print_timeout() {
    println!("\nTime is up, the exam is now over");
}

pub fn unexpected_error(e: Error) {
    println!("An unexpected error occurred during the exam: {}", e);
    println!(
        "If you see this message, you have {} failed the exam.",
        "NOT".red()
    );
    println!("Ignore any messages that follow that suggest failure");
}

pub fn no_more_questions() {
    println!("There are no more questions that you can attempt in this exam");
    println!("The exam will now end");
}

pub fn you_can_start() {
    println!(
        "You can now work on your assignment. When you are sure you're done with it, \
        use the \"{}\" command to be graded",
        "grademe".green()
    );
}

fn print_time_info(time_info: &utils::TimeInfo) {
    println!(
        "The end date for this exam is: {}",
        time_info
            .end_time
            .format("%d/%m/%Y %H:%M:%S")
            .to_string()
            .green()
    );
    let time_remaining = Instant::now().duration_since(time_info.start);
    print!("You have ");
    duration_print(&time_remaining);
    println!(" remaining");
}

pub fn exam_status(config: &Config, user: &User, exam: &Exam, time_info: &utils::TimeInfo) {
    if let Some(question) = user.current_question() {
        print_divider_bar();
        println!(
            "You are currently at level {}",
            user.level().to_string().green()
        );
        println!(
            "Your current grade is {}/{}",
            user.points().to_string().green(),
            exam.max_grade()
        );
        user.print_history();
        print_directory_info(config, question);
        print_time_info(time_info);
        print_divider_bar();
    }
}

pub fn report_exam_result(user: &User, exam: &Exam) {
    println!("You have completed the exam {}", exam.name().green());
    if user.points() >= exam.pass_grade() {
        println!(
            "You scored ({}/{})",
            user.points().to_string().green(),
            exam.max_grade()
        );
        println!("You have {} the exam", "PASSED".green());
    } else {
        println!(
            "You scored ({}/{})",
            user.points().to_string().red(),
            exam.max_grade()
        );
        println!("You have {} the exam", "FAILED".red());
    }
}
