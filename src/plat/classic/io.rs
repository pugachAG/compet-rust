use std::{fmt::Debug, io::BufRead, str::FromStr};

use crate::utils::io::{InputSource, OutputTarget};
use crate::utils::sync::Pipe;

#[macro_export]
macro_rules! read_value {
    ($io:ident, [$type:tt; $len:expr]) => {
        (0..$len).map(|_| $crate::read_value!($io, $type)).collect::<Vec<_>>()
    };
    ($io:ident, ($($type:tt),+)) => {
        ($( $crate::read_value!($io, $type), )*)
    };
    ($io:ident, usize1) => {
        $crate::read_value!($io, usize) - 1
    };
    ($io:ident, Str) => {
        crate::types::str::Str::from($crate::read_value!($io, String))
    };
    ($io:ident, $type:ty) => {
        $io.reader.read::<$type>()
    }
}

/// # Examples
/// ```
/// input! { io =>
///     (n, m): usize,
///     a: [[i32; m]; n]
/// }
/// input! { io =>
///     (n, m): usize,
///     edges: [(usize1, usize1); m]
/// }
/// ```
#[macro_export]
macro_rules! input {
    ($io:ident => ) => {};
    ($io:ident => $var:ident: $type:tt) => {
        let $var = $crate::read_value!($io, $type);
    };
    ($io:ident => ($($var:ident),+): $type:tt) => {
        $( $crate::input!{ $io => $var: $type } )*
    };
    ($io:ident => $($vars:tt: $type:tt),+) => {
        $( $crate::input!{ $io => $vars: $type } )*
    }
}

/// # Examples
/// ```
/// let (n, m) = (1, 2);
/// let a = vec![1, 2, 3];
/// let ans = false;
/// let opt: Option<usize> = None;
/// output! { io =>
///     n, m;
///     sl(a);
///     yn(ans);
///     or1(opt);
/// }
/// ```
#[macro_export]
macro_rules! output {
    ($io:ident => ) => {};
    ($io:ident => ;) => {
        $io.printer.print(&'\n');
    };
    ($io:ident => ,) => {
        $io.printer.print(&' ');
    };
    ($io:ident => $val:expr) => {
        $io.printer.print(&$val);
    };
    ($io:ident => or1 $opt:tt $($tail:tt)*) => {
        if let Some(v) = $opt {
            $io.printer.print(&v);
        } else {
            $io.printer.print(&-1);
        }
        $crate::output!{ $io => $($tail)* }
    };
    ($io:ident => sl $val:tt $($tail:tt)*) => {
        $io.printer.print_vec(&$val, ' ');
        $crate::output!{ $io => $($tail)* }
    };
    ($io:ident => nl $val:tt $($tail:tt)*) => {
        $io.printer.print_vec(&$val, '\n');
        $crate::output!{ $io => $($tail)* }
    };
    ($io:ident => yn $val:tt $($tail:tt)*) => {
        #[allow(unused_parens)]
        $io.printer.print(&(if $val { "Yes" } else { "No" }));
        $crate::output!{ $io => $($tail)* }
    };
    ($io:ident => YN $val:tt $($tail:tt)*) => {
        #[allow(unused_parens)]
        $io.printer.print(&(if $val { "YES" } else { "NO" }));
        $crate::output!{ $io => $($tail)* }
    };
    ($io:ident => $head:tt $($tail:tt)*) => {
        $crate::output!{ $io => $head }
        $crate::output!{ $io => $($tail)* }
    }
}

pub struct Io {
    pub reader: InputReader,
    pub printer: OutputPrinter,
}

impl Io {
    pub fn from_env() -> Self {
        Self::new(InputSource::from_env(), OutputTarget::from_env())
    }

    pub fn pipe() -> (Self, Self) {
        let (pipe1, pipe2) = (Pipe::new(), Pipe::new());
        (
            Self::new(
                InputSource::from_pipe(pipe1.clone()),
                OutputTarget::from_pipe(pipe2.clone()),
            ),
            Self::new(
                InputSource::from_pipe(pipe2),
                OutputTarget::from_pipe(pipe1),
            ),
        )
    }

    fn new(source: InputSource, target: OutputTarget) -> Self {
        Self {
            reader: InputReader {
                source,
                splitter: WhitespaceSplitter::default(),
            },
            printer: OutputPrinter { target },
        }
    }
}

pub struct InputReader {
    source: InputSource,
    splitter: WhitespaceSplitter,
}

pub struct OutputPrinter {
    target: OutputTarget,
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

impl InputReader {
    pub fn read<T: Parsable>(&mut self) -> T {
        self.splitter.parse_next(self.source.reader())
    }
}

impl OutputPrinter {
    pub fn print<T: ToString>(&mut self, v: &T) {
        self.target
            .writer()
            .write(v.to_string().as_bytes())
            .unwrap();
    }

    pub fn print_vec<T: ToString>(&mut self, a: &Vec<T>, sep: char) {
        let mut is_first = true;
        for item in a {
            if !is_first {
                self.print(&sep)
            }
            is_first = false;
            self.print(item)
        }
    }

    pub fn flush(&mut self) {
        let _ = self.target.writer().flush();
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
