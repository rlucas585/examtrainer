use super::YesNoAnswer::Yes;
use crate::config::Config;
use crate::exam::ExamDB;
use crate::output;
use crate::question::QuestionDB;
use crate::Error;

pub fn run(config: Config, questions: QuestionDB, exams: ExamDB) -> Result<(), Error> {
    let mut input;
    super::create_standard_directories(&config)?;

    loop {
        output::main_menu_prompt();
        input = super::read_input()?;

        let args = input.as_str().split(' ').collect::<Vec<&str>>();
        let args = args.as_slice();

        match args {
            ["list", "questions"] => print!("{}", questions),
            ["list", "exams"] => print!("{}", exams),
            ["question", name] => super::single_question::run(&config, *name, &questions)?,
            ["exam", name] => super::exam::run(&config, *name, &questions, &exams)?,
            ["config"] => output::print_config_info(&config),
            ["help"] => output::main_menu_help(),
            ["clear"] => output::clear_screen()?,
            ["exit"] | ["quit"] => return exit(&config),
            _ => output::unrecognised_command(&input),
        }
    }
}

fn exit(config: &Config) -> Result<(), Error> {
    println!("\nWould you like to delete the subject & answer directories? (y/n)? ");
    println!("The following directories would be deleted:");
    println!("- {}", config.subject_dir());
    println!("- {}", config.submit_dir());
    let answer = super::ask_yes_or_no()?;
    if matches!(answer, Yes) {
        crate::utils::delete_directory(config.submit_dir())?;
        crate::utils::delete_directory(config.subject_dir())?;
    }
    Ok(())
}
