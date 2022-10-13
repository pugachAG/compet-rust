use std::fmt::Debug;
use std::{io::BufRead, str::FromStr};

use crate::utils::io::InputSource;


#[derive(Default)]
pub struct Io {
    pub reader: InputReader,
}

pub struct InputReader {
    source: InputSource<'static>,
    splitter: WhitespaceSplitter,
}

pub trait Parsable {
    fn parse(s: &str) -> Self;
}

impl<T> Parsable for T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    fn parse(s: &str) -> Self {
        T::from_str(s).unwrap()
    }
}

impl Default for InputReader {
    fn default() -> Self {
        InputReader {
            source: InputSource::from_env(),
            splitter: WhitespaceSplitter::default(),
        }
    }
}

impl InputReader {
    pub fn read<T: Parsable>(&mut self) -> T {
        self.splitter.parse_next(self.source.reader())
    }
}

#[derive(Default)]
pub struct WhitespaceSplitter {
    buf: Vec<u8>,
}

impl WhitespaceSplitter {
    pub fn parse_next<T: Parsable>(&mut self, buf_read: &mut dyn BufRead) -> T {
        self.buf.clear();
        while let Some(byte) = consume_next_byte(buf_read) {
            match byte {
                b'\r' => {}
                b' ' | b'\n' => {
                    if !self.buf.is_empty() {
                        break;
                    }
                }
                symbol => {
                    self.buf.push(symbol);
                }
            }
        }
        if self.buf.is_empty() {
            panic!("Read nothing")
        }
        let s = std::str::from_utf8(&self.buf).unwrap();
        T::parse(s)
    }
}

fn consume_next_byte(buf_read: &mut dyn BufRead) -> Option<u8> {
    if let Some(&byte) = buf_read.fill_buf().unwrap().get(0) {
        buf_read.consume(1);
        Some(byte)
    } else {
        None
    }
}
