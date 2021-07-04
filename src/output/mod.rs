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

pub fn intro() {
    let intro = "
_____  __   _   __  __ _____ ___    _   ___ _  _ ___ ___ 
| __\\ \\/ /  /_\\ |  \\/  |_   _| _ \\  /_\\ |_ _| \\| | __| _ \\
| _| >  <  / _ \\| |\\/| | | | |   / / _ \\ | || .` | _||   /
|___/_/\\_\\/_/ \\_\\_|  |_| |_| |_|_\\/_/ \\_\\___|_|\\_|___|_|_\\
";
    println!("{}", intro.green());

    println!("Welcome to Examtrainer\n");
    println!(
        "This program was initially designed as a tool to practice for exams within the 42\
        Curriculum, but it can be used to practice just any basic programming exercises.\n"
    );

    println!(
        "You are currently at the main menu, type \"help\" to see a list of possible commands\n"
    );
}

pub fn main_menu_help() {
    println!("Possible commands are:");
    println!(
        "  {} - List all questions currently loaded by Examtrainer",
        "questions".green()
    );
    println!(
        "  {} - List all exams currently loaded by Examtrainer",
        "exams".green()
    );
    println!("  {} - Clear the terminal screen", "clear".green());
    println!("  {} - Show these commands", "help".green());
    println!("  {} - Exit examtrainer", "quit".green());
}

pub fn main_menu_prompt() {
    print!("{}> ", "examshell-admin".yellow());
    let _ = io::stdout().flush();
}

pub fn unrecognised_command(command: &str) {
    println!("Unrecognised command: '{}' ", command);
}
