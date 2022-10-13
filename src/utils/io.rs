use std::{io::{StdinLock, stdin, BufRead, BufReader}, fs::File, env};

pub enum InputSource<'a> {
    Stdin {
        stdin: StdinLock<'a>
    },
    File {
        reader: BufReader<File>,
    }
}

impl InputSource<'_> {
    pub fn from_env() -> Self {
        if env::var("LOCAL").is_ok() {
            Self::file()
        } else {
            Self::stdin()
        }
    }

    pub fn stdin() -> Self {
        InputSource::Stdin {
            stdin: stdin().lock()
        }
    }

    pub fn file() -> Self {
        let file = File::open("input.txt").unwrap();
        InputSource::File {
            reader: BufReader::new(file)
        }
    }

    pub fn reader(&mut self) -> &mut dyn BufRead {
        match self {
            InputSource::Stdin { stdin } => stdin,
            InputSource::File { reader } => reader,
        }
    }
}