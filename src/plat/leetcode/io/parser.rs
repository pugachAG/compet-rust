#[derive(Debug)]
pub enum LeetcodeValueNode {
    Int(i64),
    Str(String),
    Bool(bool),
    Array(Vec<LeetcodeValueNode>),
}

impl LeetcodeValueNode {
    pub fn parse_node(s: &str) -> Self {
        let mut rest = s;
        let node = parse_next(&mut rest);
        rest = rest.trim();
        if !rest.is_empty() {
            panic!("Unexpected {rest}");
        }
        node
    }
}

impl ToString for LeetcodeValueNode {
    fn to_string(&self) -> String {
        match self {
            LeetcodeValueNode::Int(v) => v.to_string(),
            LeetcodeValueNode::Str(s) => s.to_string(),
            LeetcodeValueNode::Bool(v) => v.to_string(),
            LeetcodeValueNode::Array(a) => format!(
                "[{}]",
                a.iter()
                    .map(|el| el.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

fn parse_next(s: &mut &str) -> LeetcodeValueNode {
    trim_start(s);
    match s.chars().next().unwrap() {
        '0'..='9' => parse_next_int(s),
        't' | 'f' => parse_next_bool(s),
        '"' => parse_next_str(s),
        '[' => parse_next_array(s),
        other => panic!("Unexpected char {other}"),
    }
}

fn parse_next_int(s: &mut &str) -> LeetcodeValueNode {
    let mut v = 0;
    while let Some(ch) = s.chars().next() {
        if let Some(d) = ch.to_digit(10) {
            v = 10 * v + d as i64;
            consume_next_char(s);
        } else {
            break;
        }
    }
    LeetcodeValueNode::Int(v)
}

fn parse_next_str(s: &mut &str) -> LeetcodeValueNode {
    let mut v = String::new();
    consume_next_char(s);
    loop {
        if let Some(ch) = consume_next_char(s) {
            if ch == '"' {
                break;
            } else {
                v.push(ch);
            }
        } else {
            panic!("Failed to find matching \"");
        }
    }
    LeetcodeValueNode::Str(v)
}

fn parse_next_bool(s: &mut &str) -> LeetcodeValueNode {
    let v = if s.starts_with("true") {
        true
    } else if s.starts_with("false") {
        false
    } else {
        panic!("Can't parse bool from {s}");
    };
    LeetcodeValueNode::Bool(v)
}

fn parse_next_array(s: &mut &str) -> LeetcodeValueNode {
    let mut res = Vec::new();
    consume_next_char(s);
    trim_start(s);
    loop {
        if let Some(ch) = s.chars().next() {
            match ch {
                ']' => {
                    consume_next_char(s);
                    break;
                }
                ',' => {
                    consume_next_char(s);
                }
                _ => {
                    res.push(parse_next(s));
                }
            }
            trim_start(s);
        } else {
            panic!("Unmatched [");
        }
    }
    LeetcodeValueNode::Array(res)
}

fn trim_start(s: &mut &str) {
    *s = s.trim_start();
}

fn consume_next_char(s: &mut &str) -> Option<char> {
    let res = s.chars().next();
    *s = &s[1..];
    res
}
