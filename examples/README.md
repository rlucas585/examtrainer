## Examtrainer configuration files

This `examples/` folder contains examples of valid config files,
question files, and exam files.

### Config files

A valid `config.toml` file **must** contain:
* A path to a submission directory (`submit_directory`)
* A path to a subjects directory (`subject_directory`)
* A path to a trace directory (`trace_directory`)
* A path to a directory containing questions (`question_directory`)
* A path to a directory containing exams (`exam_directory`)

A valid `config.toml` file **may** contain:
* A list of installed unit-test frameworks, with each containing a list
  of the required flags to compile correctly.

For an example of a valid `config.toml` file, checkout the example
[here](config.toml).

### Questions

Questions should:
* Be contained within their own directory, with all files associated
  with the question (subjects, expected output, etc.).
* Have a **single** and **valid** `.toml` file in their directory.

