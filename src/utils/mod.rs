pub mod program_output;
pub mod range;
pub mod time_info;

pub use program_output::ProgramOutput;
pub use range::Range;
pub use time_info::TimeInfo;

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

pub fn seconds_to_hours_and_minutes(mut seconds: u64) -> (u64, u64, u64) {
    let hours = seconds / 3600;
    seconds %= 3600;
    let minutes = seconds / 60;
    seconds %= 60;
    (hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_test() {
        println!("{}", timestamp());
    }
}
