use std::io::BufRead;

macro_rules! impl_incremental_parse {
    ($int_type:ty, $signed:literal) => {
        {
            let mut res: $int_type = 0;
            let mut is_neg = false;
            next_token($buf_read, |byte| {
                match byte {
                    b'0'..=b'9' => {
                        let d = (byte - b'0') as $int_type;
                        res = 10*res + d;
                    },
                    b'-' => {
                        if !$signed {
                            panic!("{:?} is not a signed type", $int_type);
                        }
                        if is_neg {
                            panic!("Multiple '-'");
                        }
                        is_neg = true;
                    },
                    other => panic!("Unexpected symbol {}", other)
                }
            });
            res
        }
    }
}

pub trait IncrementalParse {
    type Type;
    fn parse<B: BufRead>(buf_read: &mut B) -> Self::Type;
}

pub fn next_i32<B: BufRead>(buf_read: &mut B) -> i32 {
    next_integer!(buf_read, i32)
}

fn next_token<B: BufRead, F>(buf_read: &mut B, mut consume: F) where F: FnMut(u8) -> () {
    let mut read_any = false;
    while let Some(byte) = next_byte(buf_read) {
        match byte {
            b'\r' => {},
            b' ' | b'\n' => {
                if read_any {
                    break;
                }
            }
            symbol => {
                read_any = true;
                consume(symbol);
            },
        }
    }
    if !read_any {
        panic!("Read nothing")
    }
}

fn next_byte<B: BufRead>(buf_read: &mut B) -> Option<u8> {
    if let Some(&byte) = buf_read.fill_buf().unwrap().get(0) {
        buf_read.consume(1);
        Some(byte)
    } else { None }
}
