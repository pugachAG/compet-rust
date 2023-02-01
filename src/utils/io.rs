use std::{
    env,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Stdin, Stdout, Write},
};

use super::sync::Pipe;

pub enum InputSource {
    Stdin { reader: BufReader<Stdin> },
    File { reader: BufReader<File> },
    Pipe { reader: BufReader<Pipe> },
}

pub enum OutputTarget {
    Stdout { writer: BufWriter<Stdout> },
    File { writer: BufWriter<File> },
    Pipe { pipe: Pipe },
}

impl InputSource {
    pub fn from_env() -> Self {
        if is_local() {
            Self::from_file()
        } else {
            Self::from_stdin()
        }
    }

    pub fn from_stdin() -> Self {
        InputSource::Stdin {
            reader: BufReader::new(stdin()),
        }
    }

    pub fn from_file() -> Self {
        let file = File::open("input.txt").unwrap();
        InputSource::File {
            reader: BufReader::new(file),
        }
    }

    pub fn from_pipe(pipe: Pipe) -> Self {
        InputSource::Pipe {
            reader: BufReader::new(pipe),
        }
    }

    pub fn reader(&mut self) -> &mut dyn BufRead {
        match self {
            InputSource::Stdin { reader } => reader,
            InputSource::File { reader } => reader,
            InputSource::Pipe { reader } => reader,
        }
    }
}

impl OutputTarget {
    pub fn from_env() -> Self {
        if is_local() {
            Self::from_file()
        } else {
            Self::from_stdout()
        }
    }

    pub fn from_stdout() -> Self {
        OutputTarget::Stdout {
            writer: BufWriter::new(stdout()),
        }
    }

    pub fn from_file() -> Self {
        OutputTarget::File {
            writer: BufWriter::new(File::create("out/output.txt").unwrap()),
        }
    }

    pub fn from_pipe(pipe: Pipe) -> Self {
        OutputTarget::Pipe { pipe }
    }

    pub fn writer(&mut self) -> &mut dyn Write {
        match self {
            OutputTarget::Stdout { writer } => writer,
            OutputTarget::File { writer } => writer,
            OutputTarget::Pipe { pipe } => pipe,
        }
    }
}

fn is_local() -> bool {
    env::var("LOCAL").is_ok()
}
