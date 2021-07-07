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
  of the required flags to compile correctly. (`frameworks`)

For an example of a valid `config.toml` file, checkout the example
[here](config.toml).

### Questions

Questions should:
* Be contained within their own directory, with all files associated
  with the question (subjects, expected output, etc.).
* Have a **single** and **valid** `.toml` file in their directory.

An example of a valid question can be found [here](hello/)

#### Question Directory layout

An example of a valid question directory would look something like this:
```
hello
├── hello.err
├── hello.out
├── hello.subject               # REQUIRED
│  └── subject.en.txt
├── hello.toml                  # REQUIRED
└── main.c
```

In the directory above:
* `hello` is the directory name, and while it does not need to
  have the same name as the question itself (as outlined in the `.toml`
  file), it is good practice to do so.
* `hello.toml` is the question's config file. As with the directory
  name, it does not need to have the same name as the question, but it
  is good practice to do so. You MUST NOT have more than one `.toml`
  file in the question directory.
* `hello.subject` is a directory containing subject files. These will be
  used to explain the question to users, so they should be clear and
  ideally, in multiple languages.
* `hello.out`, `hello.err` are present due to this question being tested
  with the `expected-output` test type. These files should contain the
  expected outputs (to stdout and stderr) of the submission when
  compiled.
* `main.c` is present also due to the question being an `expected-output`
  test type. This file will be compiled together with the files
  submitted by the user to generate an executable, which will then be
  tested against the expected output in `hello.out`.

#### Question .toml file

For an example of a valid `.toml` file, see
[here](hello/hello.toml).

##### Info section

The `info` table is used to provide meta-information about the question,
and crucially the **name** of the question.

The `info` table contains the following fields:
* `name` - (**Required**) The name of the question. This is crucial to
  identify the question, and no two questions should have the same name.
* `description` - (**Optional**) A brief, one sentence description of
  the question. This is not required, but it is good practice to write
  one.
* `authors` - (**Optional**) Author/Authors of the question.
* `difficulty` - (**Optional**) How difficult the question is, on a
  scale of 0-100.

###### Example

```
[info]
name = "hello"
description = "Write a Hello World! function"
authors = [
    "Ryan Lucas"
]
difficulty = 2
```

##### Submission section

The `submission` table provides info about how the user should submit
their answer.

There are two different types of submission possible:
* `executable` - This submission type asks that the user compiles their
  executable themselves.
* `sources` - This submission type requires the user to submit specific
  source files, that will then be compiled by Examtrainer (either
  individually or with other files) for testing.

The `submission` table contains the following fields:
* `submission_type` - (**Required**) Must be either `sources` or
  `executable`.
* `sources` - (**Required for `sources` type**) A list of source files
  that the user must submit.

###### Example

```
[submission]
submission_type = "sources"
sources = ["hello_world.c"]
```

##### Test section

The `test` table provides info about how the submitted code will be
tested.
Examtrainer tries to be very flexible in accommodating multiple
different methods of testing - the downside of this is that there are
multiple different `test_type`'s (currently 4), all with different
requirements.

The different test types will be covered one by one.

###### Test Types

* `test_type` - Can be one of 4 different types:
  - `expected-output` - Compiles test source files together with user
  submitted source files, then runs the resulting executable with
  arguments described in the `.toml` file. The output is then compared
  against expected output files, and the test is passed if output is as
  expected.
  - `sources` - Compiles test source files **separately** from the
  user's submitted source files to produce **two** binaries. The
  binaries are then run with the same arguments, and the output of both
  binaries are compared. The test is passed if all output is identical.
  - `executable` - Compiles user submitted source files into a binary,
  then compares the output of the user-binary with output from the test
  executable. The test is passed if output from both binaries is
  identical.
  - `unit-test` - Compiles user submitted source files together with a
  unit test. The resulting binary is run, and the test is passed if the
  executable exits with an exit code of 0.
  Unit tests can be created without any external frameworks, but
  Examtrainer does offer the option of including them.

###### Expected Output Test Type

Required fields:
* `sources` - An array of source files contained within the question
  directory, to be compiled with user submitted sources. This can be
  empty if the user is asked to submit a program, but it still must be
  present.
* `compiler` - The compiler required to generate an executable from the
  source files. Must not conflict with the compiler listed in the
  submission table (if it is supplied).
* `expected-stdout` - A text file containing the output to stdout that
  the produced executable should generate when run with the arguments
  listed under the `args` field.
* `expected-stderr` - A text file containing the output to stderr that
  the executable should generate.
* `args` - An array of arrays, containing the different arguments that
  the executable should be run with the produce the required output.
* `subject` - The path of the directory with the subject
  files.

Optional fields:
* `flags` - Flags to be used during the compilation stage. For any C
  question, "-Wall -Wextra -Werror" is recommended, as 42 use these flags
  during compilation.

```
[test]
test_type = "expected-output"
sources = []
subject = "only_a.subject"
expected_stdout = "only_a.out"
expected_stderr = "only_a.err"
compiler = "gcc"
flags = ["-Wall", "-Wextra", "-Werror"]
args = [
 [],
 [ "args", "do", "nothing", "for", "this", "question" ],
]
```

###### Sources Test Type

Required fields:
* `sources` - An array of source files to be compiled **separately**
  from the user's sources.
* `compiler` - The compiler used to compile the test source files.
* `args` - An array of arrays, containing the different arguments that
  both the user and test executables will use when running.
* `subject` - The path of the directory with the subject
  files.

Optional fields:
* `flags` - Flags to be used during compilation. Less important than in
  `expected-output`, as they are used exclusively for the test files,
  which should not fail to compile.

```
[test]
test_type = "sources"
sources = ["ft_countdown.c"]
compiler = "gcc"
subject = "ft_countdown.subject"
args = [
    [],
    ["I'll", "be", "ignored", "in", "this", "question"],
]
```

###### Executable Test Type

Required fields:
* `binary` - Path to a binary contained with the question, which the
  user should be replicating with their submission.
* `args` - An array of arrays, containing the different arguments that
  both the user and test executables will use when running.

```
[test]
test_type = "executable"
binary = "ft_countdown"
subject = "ft_countdown.subject"
expected_stdout = "ft_countdown.out"
expected_stderr = "ft_countdown.err"
args = [
    [],
    ["I'll", "be", "ignored"],
]
```

###### Unit Test Test Type

Required fields:
* `compiler` - The compiler used to compile the Unit Test with the user
  submitted files.
* `sources` - The Unit Test source files.

Optional fields:
* `flags` - Flags to be used during compilation.
* `framework` - If the Unit Test uses an external testing framework,
  such as GoogleTest or Catch2, then it should be named here.
  Additionally, this will require the framework to be listed in the
  `config.toml` file of Examtrainer.

```
[test]
test_type = "unit-test"
sources = ["unit_test.cpp"]
flags = ["-Wall", "-Wextra", "-Werror"]
compiler = "g++"
framework = "gtest"
subject = "ft_strlen.subject"
```
