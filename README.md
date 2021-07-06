# Examtrainer **Alpha Release**

## About The Project

This project aims to:
1. Simulate the `examshell` environment from the 42 school, allowing 42
   students (or anyone else who's interested) to practice the exams in
   the 42 curriculum and Piscines.
2. Allow users to add their own questions, and create their own exams.

This project is currently in an **Alpha Release**. The program runs, but
testing to date is minimal.

### Built With

* [toml-rs](https://github.com/alexcrichton/toml-rs)
* [serde](https://github.com/serde-rs/serde)
* [home](https://github.com/brson/home)
* [term_size](https://github.com/clap-rs/term_size-rs)
* [colored](https://github.com/mackwic/colored)
* [chrono](https://github.com/chronotope/chrono)
* [rand](https://github.com/rust-random/rand)
* [crossbeam](https://github.com/crossbeam-rs/crossbeam)

## Installation

#### Build from Sources

To build `examtrainer` from sources, Rust must be installed. The
instructions to install Rust can be found
[here](https://www.rust-lang.org/tools/install).

1. Clone the project `git clone https://github.com/rlucas585/examtrainer`.
2. [Create configuration (IN DEVELOPMENT)]
3. Run `cargo build --release` to build the executable from sources.
4. The executable will be located at `target/release/examtrainer`. It
   can be moved from here to wherever you'd like.

## Configuration

`examtrainer` requires a configuration file to run, which must be in the
[TOML](https://github.com/toml-lang/toml) file format.

`examtrainer` will by default use a config file located at
`/home/<your_username>/.config/examtrainer/config.toml`. If this path
does not exist, then the first time that `examtrainer` is run it will
ask to create the relevant directories, and will create a default
`config.toml` file.

Alternatively, you can supply a path to a configuration file as a
command line argument:
```
./examtrainer <path_to_config_file>
```

Config files, Question files and Exam files are all `.toml` files, and
the files must contain certain information to be valid. The required
layout of these files is described [here](examples/), alongside examples
of valid files.

## Usage

[IN DEVELOPMENT]

## RoadMap

[IN DEVELOPMENT]

## Author

- Ryan Lucas (ryanl585codam@gmail.com)
