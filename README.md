# examtrainer

## About The Project

This is the `develop` branch - `main` can be seen for an explanation of
the goal of the project. `develop` aims to reach the same level of
functionality as `main`, but with much cleaner code, before moving
beyond and incorporating new features.

### Built With

* [toml-rs](https://github.com/alexcrichton/toml-rs)
* [serde](https://github.com/serde-rs/serde)
* [home](https://github.com/brson/home)
* [term_size](https://github.com/clap-rs/term_size-rs)
* [colored](https://github.com/mackwic/colored)
* [chrono](https://github.com/chronotope/chrono)

## Installation & Usage

(Unavailable until alpha-release).

## RoadMap

* Load Questions and Exams from Configuration directories, printing
  Warning messages for Questions/Exams that are invalid.
* Create an `examshell-admin` shell mode, where questions or exams can
  be listed, and an exam can be begun.
  - A long term goal is for some sort of package manager to also be
  built in, to install external questions/exams from an online source.
  For the time being, manual installation is acceptable.
* Implement grading for all types of submission, and tests (e.g. allow
  submission of an executable, or of source files).
* Run the `examshell` in a thread with a timeout based on the time
  allowed for the exam.
* Another long term goal is to save a user's history with examinations,
  but this is not a priority.

## Author

- Ryan Lucas (ryanl585codam@gmail.com)
