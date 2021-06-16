#!/bin/bash

# Creates a new generic question template

if [ "$#" -ne 1 ]; then
    echo "Please supply a name for the new question"
    exit
fi
if [ -d "$1" ]; then
    echo "Question '$1' is already present in the current directory"
    exit
fi

DEFAULT_TOML="[info]\n\
name = \"$1\"\n\
authors = [\n\
    \"XXX\"\n\
]\n\
difficulty = XXX # Scale from 0-100\n\
\n\
[submission]\n\
submission_type = \"sources\"\n\
sources = [\"$1.c\"]\n\
\n\
[test]\n\
test_type = \"expected-output\"\n\
sources = []\n\
subject = \"$1.subject\"\n\
expected_stdout = \"$1.out\"\n\
expected_stderr = \"$1.err\"\n\
compiler = \"gcc\"\n\
flags = [\"-Wall\", \"-Wextra\", \"-Werror\"]\n\
args = [\n\
    [],\n\
]\n\
"

mkdir $1
mkdir $1/$1.subject
touch $1/$1.out
touch $1/$1.err
touch $1/$1.subject/subject.en.txt
echo -e $DEFAULT_TOML > $1/$1.toml
