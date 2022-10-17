use std::{
    env,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, StdinLock, Stdout, Write},
};

pub enum InputSource<'a> {
    Stdin { stdin: StdinLock<'a> },
    File { reader: BufReader<File> },
}

pub enum OutputTarget {
    Stdout { writer: BufWriter<Stdout> },
    File { writer: BufWriter<File> },
}

impl InputSource<'_> {
    pub fn from_env() -> Self {
        if is_local() {
            Self::from_file()
        } else {
            Self::from_stdin()
        }
    }

    pub fn from_stdin() -> Self {
        InputSource::Stdin {
            stdin: stdin().lock(),
        }
    }

    pub fn from_file() -> Self {
        let file = File::open("input.txt").unwrap();
        InputSource::File {
            reader: BufReader::new(file),
        }
    }

    pub fn reader(&mut self) -> &mut dyn BufRead {
        match self {
            InputSource::Stdin { stdin } => stdin,
            InputSource::File { reader } => reader,
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

    pub fn writer(&mut self) -> &mut dyn Write {
        match self {
            OutputTarget::Stdout { writer } => writer,
            OutputTarget::File { writer } => writer,
        }
    }
}

fn is_local() -> bool {
    env::var("LOCAL").is_ok()
}
