use crate::Error;
use std::io::{self, Read, Write};

pub fn read_input() -> Result<String, Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer)?;
    let first_whitespace = buffer.find(|c: char| c.is_whitespace());
    if let Some(trim_point) = first_whitespace {
        buffer.truncate(trim_point);
    }
    Ok(buffer)
}

pub fn clear_screen() -> Result<(), Error> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().map_err(|e| e.into())
}
