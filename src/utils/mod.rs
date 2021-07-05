pub mod program_output;
pub mod range;

pub use program_output::ProgramOutput;
pub use range::Range;

use chrono::{Datelike, Timelike};

pub fn delete_directory(name: &str) -> Result<(), std::io::Error> {
    std::process::Command::new("rm")
        .arg("-r")
        .arg(name)
        .output()?;
    Ok(())
}

pub fn timestamp() -> String {
    let now = chrono::Local::now();
    format!(
        "{:02}:{:02}:{:02}-{}_{}_{}",
        now.hour(),
        now.minute(),
        now.second(),
        now.day(),
        now.month(),
        now.year(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_test() {
        println!("{}", timestamp());
    }
}
