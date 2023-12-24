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
        state: (0..n).collect(),
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
