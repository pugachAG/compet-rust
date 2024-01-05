use std::time::Instant;

use crate::math::combinations::combinations;
use crate::math::combinatorics::combinations_count;
use crate::types::default_dict::DefaultDict;
use crate::utils::rand::Random;

use super::includes::*;

exec_mode!(multitest = false);

fn remedian(a: &[usize]) -> usize {
    let n = a.len();
    if n > 1 {
        assert!(a.len() % 3 == 0);
        let m = n / 3;
        let b = [
            remedian(&a[0..m]),
            remedian(&a[m..2 * m]),
            remedian(&a[2 * m..]),
        ];
        b.sum() - b.max_element() - b.min_element()
    } else {
        a[0]
    }
}

fn monte_carlo(n: usize, iters: usize) -> Vec<f64> {
    let mut a = (0..n).into_vec();
    let mut pos = vec![0usize; n];
    let mut rand = Random::new(42);
    for _ in 0..iters {
        rand.shuffle(&mut a);
        pos[remedian(&a)] += 1;
    }
    pos.into_iter().map(|c| c as f64 / iters as f64).into_vec()
}

fn median(a: usize, b: usize, c: usize) -> usize {
    let mx = a.max(b).max(c);
    let mn = a.min(b).min(c);
    return a + b + c - mx - mn;
}

fn fill_rem(target: &mut [usize], c: &[usize], rem: &[usize]) {
    let mut i_c = 0;
    let mut i = 0;
    for &v in rem {
        if i_c < c.len() && c[i_c] == v {
            i_c += 1;
        } else {
            target[i] = v;
            i += 1;
        }
    }
    assert_eq!(i, target.len());
    assert_eq!(i_c, c.len());
}

fn count_slow(n: usize) -> Vec<u128> {
    if n == 1 {
        return vec![1];
    }
    let prev = count_slow(n / 3);
    let relevant = (0..n/3).filter(|&i| prev[i] != 0).into_vec();
    let mut ans = vec![0u128; n];
    let mut combs = combinations(n, n / 3);
    let mut cur_step = 0;
    let tot_steps = combinations_count(n, n / 3);
    let start = Instant::now();
    let mut c3 = vec![0; n / 3];
    let mut c2 = vec![0; n / 3];
    let mut rem = vec![0; 2*n / 3];
    let all = (0..n).into_vec();
    while let Some(c1) = combs.next() {
        cur_step += 1;
        if cur_step % 100 == 0 {
            let remaining = start.elapsed() * (tot_steps / cur_step) as u32;
            eprint!("\rprogress for {n}: {:.2}% remaining {} minutes",
                100.0 * cur_step as f64 / tot_steps as f64,
                remaining.as_secs() / 60
            );
        }
        fill_rem(&mut rem, c1, &all);
        let mut combs = combinations(rem.len(), n / 3);
        while let Some(comb2) = combs.next() {
            for i in 0..c2.len() {
                c2[i] = rem[comb2[i]];
            }
            fill_rem(&mut c3, &c2, &rem);
            for &i1 in relevant.iter() {
                for &i2 in relevant.iter() {
                    let remed = c2[i2];
                    if c1[i1] > remed {
                        continue;
                    }
                    for &i3 in relevant.iter() {
                        if c3[i3] > remed {
                            let cnt = prev[i1] * prev[i2] * prev[i3];
                            ans[remed] += 6 * cnt;
                        }
                    }
                }
            }
        }
    }
    eprintln!("");
    ans
}

fn count_dbl(prev: &[u128]) -> Vec<((usize, usize), u128)> {
    let n = prev.len() * 2;
    let mut cnt = DefaultDict::<(usize, usize), u128>::new();
    let all = (0..n).into_vec();
    let mut combs = combinations(n, n / 2);
    let mut c2 = vec![0; n / 2];
    while let Some(c1) = combs.next() {
        fill_rem(&mut c2, c1, &all);
        for (i1, cnt1) in prev.iter().enumerate() {
            for (i2, cnt2) in prev.iter().enumerate() {
                cnt[(c1[i1], c2[i2])] += cnt1 * cnt2;
            }
        }
    }
    cnt.into_iter().collect()
}

fn count(n: usize) -> Vec<u128> {
    if n == 1 {
        return vec![1];
    }
    let prev = count(n / 3);
    let dbl = count_dbl(&prev);
    let all = (0..n).into_vec();
    let mut combs = combinations(n, n / 3);
    let mut other = vec![0; 2 * n / 3];
    let mut ans = vec![0u128; n];
    while let Some(c3) = combs.next() {
        fill_rem(&mut other, c3, &all);
        for &((i1, i2), cnt12) in dbl.iter() {
            let v1 = other[i1];
            let v2 = other[i2];
            for (i3, &cnt3) in prev.iter().enumerate() {
                let v3 = c3[i3];
                ans[remedian(&[v1, v2, v3])] += cnt12 * cnt3;
            }
        }
    }
    ans
}

fn prob(n: usize) -> Vec<f64> {
    let tot = (1..=(n as u128)).reduce(|a, b| a * b).unwrap() as f64;
    count(n).into_iter().map(|c| c as f64 / tot).collect()
}

pub fn solve(_io: &mut Io) {
    let n = 27;
    let actual = prob(n);
    let est = monte_carlo(n, 100_000);
    for i in 0..n {
        println!("{i}: got {}, est {}", actual[i], est[i]);
    }
}
