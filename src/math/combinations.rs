/// Usage example: `combinations_basic` test
pub fn combinations(n: usize, k: usize) -> CombinationGenerator {
    CombinationGenerator { n, k, state: None }
}

pub struct CombinationGenerator {
    n: usize,
    k: usize,
    state: Option<Vec<usize>>,
}

impl CombinationGenerator {
    pub fn next(&mut self) -> Option<&[usize]> {
        if let Some(state) = self.state.as_mut() {
            if !next_combination(state, self.n) {
                return None;
            }
        } else {
            self.state = Some((0..self.k).collect());
        }
        self.state.as_deref()
    }
}

/// `a` is expected to be sorted and contain unique numbers in 0..n range
pub fn next_combination(a: &mut [usize], n: usize) -> bool {
    let Some(start) = find_next_combination_start(a, n) else {
        return false;
    };
    a[start] += 1;
    for j in (start + 1)..a.len() {
        a[j] = a[j - 1] + 1;
    }
    true
}

fn find_next_combination_start(a: &mut [usize], n: usize) -> Option<usize> {
    let k = a.len();
    for i in (0..k).rev() {
        if a[i] != n - (k - i) {
            return Some(i);
        }
    }
    None
}
