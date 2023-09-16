pub fn modulo_pow(v: u64, exp: u64, modulo: u64) -> u64 {
    let mut ans = 1;
    let mut r = exp;
    let mut cur = v;
    while r > 0 {
        if (r & 1) == 1 {
            ans = (ans * cur) % modulo;
        }
        cur = (cur * cur) % modulo;
        r >>= 1;
    }
    ans
}

pub fn modulo_inv(v: u64, modulo: u64) -> u64 {
    assert!(v != 0, "can't calculate inverse of 0");
    modulo_pow(v, modulo - 2, modulo)
}

#[derive(Default)]
pub struct Factorials {
    modulo: u64,
    mem: Vec<u64>
}

impl Factorials {
    pub fn new(n: usize, modulo: u64) -> Self {
        let mut mem = vec![1u64; n+1];
        for i in 1..=n {
            mem[i] = (mem[i-1] * i as u64) % modulo;
        }
        Self {
            modulo,
            mem
        }
    }

    pub fn f(&self, n: usize) -> u64 {
        self.mem[n]
    }
}

pub fn modulo_combinations(fact: &Factorials, n: usize, k: usize) -> u64 {
    assert!(k <= n);
    let d = (fact.f(k) * fact.f(n-k)) % fact.modulo;
    (fact.f(n) * modulo_inv(d, fact.modulo)) % fact.modulo
}

