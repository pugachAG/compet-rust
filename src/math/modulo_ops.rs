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
