use std::cmp::min;

pub fn pair_count(n: usize) -> usize {
    n * n.saturating_sub(1) / 2
}

pub fn combinations_count(n: usize, k: usize) -> usize {
    let k = min(n - k, k);
    let mut ans = 1;
    let mut nxt = 1;
    for i in 0..k {
        ans *= n - i;
        while nxt <= k && ans % nxt == 0 {
            ans /= nxt;
            nxt += 1;
        }
    }
    for v in nxt..=k {
        ans /= v;
    }
    ans
}

pub fn factorial(n: usize) -> usize {
    (1..=n).reduce(|a, b| a * b).unwrap_or(1)
}
