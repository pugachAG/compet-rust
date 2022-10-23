#[derive(Debug)]
pub enum LeetcodeValueNode {
    Int(i64),
    Str(String),
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
    *s = s.trim_start();
    match s.chars().next().unwrap() {
        '0'..='9' => {
            let mut v = 0;
            while let Some(ch) = s.chars().next() {
                if let Some(d) = ch.to_digit(10) {
                    v = 10 * v + d as i64;
                    *s = &s[1..]
                } else {
                    break;
                }
            }
            LeetcodeValueNode::Int(v)
        }
        '"' => {
            let v: String = s[1..].chars().take_while(|&ch| ch != '"').collect();
            let n = v.len();
            if s.chars().nth(n + 1) != Some('"') {
                panic!("Failed to find matching \"");
            }
            *s = &s[n + 2..];
            LeetcodeValueNode::Str(v)
        }
        '[' => {
            let mut res = Vec::new();
            *s = &s[1..].trim_start();
            loop {
                if let Some(ch) = s.chars().next() {
                    match ch {
                        ']' => {
                            *s = &s[1..];
                            break;
                        }
                        ',' => {
                            *s = &s[1..];
                        }
                        _ => {
                            res.push(parse_next(s));
                        }
                    }
                    *s = s.trim();
                } else {
                    panic!("Unmatched [");
                }
            }
            LeetcodeValueNode::Array(res)
        }
        other => panic!("Unexpected char {other}"),
    }
}
