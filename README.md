# examtrainer **CONCEPT**

## About The Project

This project aims to:
1. Simulate the `examshell` environment from the 42 school, allowing 42
   students (or anyone else who's interested) to practice the exams in
   the 42 curriculum and piscines.
2. Allow users to add their own questions, and create their own exams.

This branch (`main`), is currently a **proof-of-concept**. The objective
was for:
* More than one exam to be possible.
* More than one question to be possible, in a single exam.
* The program to return **Success** if the question was answered
  correctly, **Failure** otherwise, when the `grademe` command was run.
* Simulate the true `examshell`'s output.
* Define configuration files for the program itself, exam questions
  (named `modules` originally, changed in `develop` branch), and exams.

All of this was achieved, but the code was gradually becoming more
unwieldy, and there are a couple of design decisions that are going to
be changed in the `develop` branch. In particular, loading of questions
and exams is going to be greatly simplified, the code's modularity
is going to be greatly improved, and there will be much more extensive
test coverage.

### Built With

* [toml-rs](https://github.com/alexcrichton/toml-rs)
* [serde](https://github.com/serde-rs/serde)
* [home](https://github.com/brson/home)
* [term_size](https://github.com/clap-rs/term_size-rs)
* [colored](https://github.com/mackwic/colored)
* [chrono](https://github.com/chronotope/chrono)

## Installation & Usage

This branch is not intended to be used currently, as the program is as
of now of an insufficient standard. If you do want to try the program
out however, it is possible, but it does require Rust to be installed
(which can be done [here](https://www.rust-lang.org/tools/install)).

1. Clone the project `git clone https://github.com/rlucas585/examtrainer`.
2. Place the `modules` directory and the `exams` directory in
   `$HOME/.config/examtrainer/`.
3. Run `cargo build` & `./target/debug/examtrainer` to run the program.
   `cargo run` will also run the program. You can also build with a
   release profile using `cargo build --release`, but it doesn't
   significantly affect performance at this stage.
4. Instructions from the `examshell` should be sufficient to sit the
   exam. (Currently however the program is hard coded to run
   `Exam_2.toml` from the `exams` directory, at this point this can only
   be altered by editing the source code directly - this is obviously
   going to be fixed in the `develop` branch.)
5. Also note that while "Time Remaining" is displayed, the exam can
   currently be sat indefinitely.

## Author

- Ryan Lucas (ryanl585codam@gmail.com)
