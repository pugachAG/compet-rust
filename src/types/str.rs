#[derive(Default, Clone)]
pub struct Str(pub Vec<u8>);

impl ToString for Str {
    fn to_string(&self) -> String {
        self.to_std()
    }
}

impl From<String> for Str {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<&str> for Str {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl Str {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(n: usize, ch: u8) -> Self {
        Self(vec![ch; n])
    }

    pub fn to_std(&self) -> String {
        self.0.iter().map(|&b| b as char).collect()
    }
}
