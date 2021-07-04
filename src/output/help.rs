use colored::*;

pub fn main_menu_help() {
    println!("Possible commands are:");
    println!(
        "  {} - List all questions currently loaded by Examtrainer",
        "list questions".green()
    );
    println!(
        "  {} - List all exams currently loaded by Examtrainer",
        "list exams".green()
    );
    println!(
        "  {} - Enter single question mode, to practice on one question",
        "question <question_name>".green()
    );
    println!("  {} - Clear the terminal screen", "clear".green());
    println!("  {} - Show these commands", "help".green());
    println!("  {} - Exit examtrainer", "quit".green());
}

pub fn single_question_help() {
    println!("Possible commands are:");
    println!(
        "  {} - List all questions currently loaded by Examtrainer",
        "list questions".green()
    );
    println!(
        "  {} - List all exams currently loaded by Examtrainer",
        "list exams".green()
    );
    println!("  {} - Clear the terminal screen", "clear".green());
    println!("  {} - Show these commands", "help".green());
    println!(
        "  {} - Exit single question mode and return to main menu",
        "quit".green()
    );
}
