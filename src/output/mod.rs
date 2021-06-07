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
    println!("  Level {}:", format!("{}", status.level).green());
    for assignment in status.assignments.iter() {
        println!("{}", assignment);
    }
    println!("");
    if let Some(last) = status.assignments.last() {
        last.print();
        println!(
            "It is assignment {} for level {}",
            format!("{}", status.attempt).yellow(),
            format!("{}", status.level).green()
        );
        // TODO: Add subject directory line in status printing
        println!(
            "You must turn in your files in a {}
with the same name as the assignment ({}).",
            "subdirectory of your submit directory".bold(),
            last.submit_location().red().bold()
        );
        println!("examtrainer does not require you to git push, but remember for real exams!\n");
    }
}

fn print_time(status: &Status) {
    let time_remaining = status.time_remaining().as_secs();
    let end = status.end_time();
    println!(
        "The end date for this exam is: {}",
        end.format("%d/%m/%Y %H:%M:%S").to_string().green()
    );
    println!(
        "You have {} remaining",
        format!(
            "{}hrs, {}mins and {}sec",
            time_remaining / 3600,
            (time_remaining % 3600) / 60,
            (((time_remaining % 3600) % 60) % 60)
        )
        .green()
    );
}

pub fn print_status(status: &Status) {
    println!("");
    print_divider_bar();
    println!(
        "You are currently at level {}",
        format!("{}", status.level).green()
    );
    println!("Your current grade is {}", status.grade);

    print_completed_assignments(status);

    print_time(status);

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
        exam.name.green()
    );
    println!(
        "You will have {} to complete this exam",
        format!("{}", exam.time).green()
    );
    println!("Time will begin once you press enter to continue\n");
    println!("(Press Enter to continue...)");
}

pub fn print_prompt() {
    print!("{}> ", "examshell".yellow()); // Maybe make bold?
    let _ = io::stdout().flush();
}

pub fn print_unrecognised(command: &str) {
    println!("Unrecognised command: '{}'", command.trim());
    print_help();
}

pub fn print_help() {
    println!("Possible commands are:");
    println!(
        "  {} - Submits and grades the current assignment",
        "grademe".green()
    );
    println!(
        "  {} - Print the current status to the screen",
        "status".green()
    );
    println!("  {} - Clear the terminal screen", "clear".green());
    println!("  {} - Show these commands", "help".green());
    println!("  {} - Exit examtrainer", "quit".green());
    print_prompt();
}
