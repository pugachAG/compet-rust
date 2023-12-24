use crate::plat::classic::includes::IntoVecExt;

pub fn pair_count(n: usize) -> usize {
    n * n.saturating_sub(1) / 2
}

pub type Mask = Vec<bool>;

pub fn generate_combinations(n: usize, k: usize) -> Vec<Mask> {
    assert!(k <= n);
    fn gen(state: &mut Mask, i: usize, cur: usize, k: usize, ans: &mut Vec<Mask>) {
        if cur == k {
            ans.push(state.clone());
        } else if i < state.len() {
            state[i] = true;
            gen(state, i + 1, cur + 1, k, ans);
            state[i] = false;
            gen(state, i + 1, cur, k, ans);
        }
    }
    let mut ans = vec![];
    let mut state = vec![false; n];
    gen(&mut state, 0, 0, k, &mut ans);
    ans
}

pub struct PermutationGenerator {
    init: bool,
    state: Vec<usize>,
}

impl PermutationGenerator {
    pub fn next(&mut self) -> Option<&[usize]> {
        if self.init {
            self.init = false;
            Some(&self.state)
        } else if next_permutation(&mut self.state) {
            Some(&self.state)
        } else {
            None
        }
    }
}

/// Usage example: `permutations_basic` test
pub fn permutations(n: usize) -> PermutationGenerator {
    PermutationGenerator {
        init: true,
        state: (0..n).into_vec(),
    }
}

pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let Some(start) = find_next_permutation_start(a) else {
        return false;
    };
    let (i, _) = a
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, v)| **v > a[start])
        .next()
        .unwrap();
    a.swap(start, i);
    a[start + 1..].reverse();
    true
}

fn find_next_permutation_start<T: Ord>(a: &[T]) -> Option<usize> {
    for (i, v) in a.iter().enumerate().rev().skip(1) {
        if *v < a[i + 1] {
            return Some(i);
        }
    }
    None
}
