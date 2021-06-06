use crate::config::exams::Exam;
use crate::Status;
use colored::*;
use std::io::{self, Read, Write};

fn print_divider_bar() {
    let (width, _) = term_size::dimensions().unwrap_or((50, 50));
    let width = if width <= 11 { 11 } else { width - 10 };
    for _ in 0..width - 1 {
        print!("=");
    }
    println!("=");
}

// TODO: Print completed assignment
fn print_completed_assignments(status: &Status) {
    println!("Assignments:");
}

pub fn print_question_intro(exam: &Exam, status: &Status) {
    print_divider_bar();
    println!(
        "You are currently at level {}",
        format!("{}", status.level).green()
    );
    println!(
        "Your current grade is {}/{}",
        format!("{}", status.points).green(),
        exam.config.pass_grade
    );

    print_divider_bar();
    println!(
        "You can now work on your assignment. When you are sure you're done with it, use the \"{}\"
command to be graded",
        "grademe".green()
    );
    print_prompt();
}

pub fn print_exam_intro(exam: &Exam) {
    println!(
        "You are registered to begin the exam: {}",
        exam.info.name.green()
    );
    println!(
        "You will have {} to complete this exam",
        format!("{}", exam.config.time).green()
    );
    println!("Time will begin once you press enter to continue\n");
    println!("(Press Enter to continue...)");
    std::io::stdin().read(&mut [0]).unwrap();
}

fn print_prompt() {
    print!("{}> ", "examshell".yellow()); // Maybe make bold?
    let _ = io::stdout().flush();
}
