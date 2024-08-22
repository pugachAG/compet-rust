/// returns vector of first divisors
pub fn eratosthenes_sieve(n: usize) -> Vec<usize> {
    let mut ret: Vec<usize> = (0..n).map(|i| i).collect();
    for p in 2..n {
        if ret[p] == p {
            for v in (2*p..n).step_by(p) {
                if ret[v] == v {
                    ret[v] = p;
                }
            }
        }
    }
    ret
}