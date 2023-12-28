use crate::ds::bitset::Bitset;

pub fn knapsack(a: &[usize], s: usize) -> bool {
    knapsack_all(a, Some(s)).get(s)
}

pub fn knapsack_all(a: &[usize], limit: Option<usize>) -> Bitset {
    let n = limit.unwrap_or_else(|| a.iter().sum::<usize>()) + 1;
    let mut bs = Bitset::new(n);
    bs.set(0, true);
    for &v in a {
        bs.shl_or(v);
    }
    bs
}
