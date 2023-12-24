pub fn pair_count(n: usize) -> usize {
    n * n.saturating_sub(1) / 2
}

pub fn factorial(n: usize) -> usize {
    (1..=n).reduce(|a, b| a * b).unwrap_or(1)
}
