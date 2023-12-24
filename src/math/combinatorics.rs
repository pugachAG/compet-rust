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
