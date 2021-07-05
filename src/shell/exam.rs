use crate::output;
use crate::user::User;
use crate::Error;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn run(val: &str) -> Result<(), Error> {
    let val = val
        .parse::<u64>()
        .map_err(|_| Error::General("parse error".to_string()))?;
    let mut user = Arc::new(Mutex::new(User::new()));
    let (sender, receiver) = mpsc::channel();

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(3));

        sender.send(true).unwrap();
    });
    match receiver.recv_timeout(Duration::from_secs(val)) {
        Ok(_) => (),
        Err(_) => output::print_timeout(),
    }
    let end_user = Arc::try_unwrap(user)
        .map_err(|_| Error::General("Thread error".to_string()))?
        .into_inner()
        .map_err(|_| Error::General("Thread error".to_string()))?;
    Ok(())
}
