use std::ops::Range;

use crate::ds::lazy_seg_tree::{LazySegTree, LazySegTreeUpdate};
use crate::ds::seg_tree::SegTreeValue;
use crate::utils::rand::Random;

#[derive(Clone, Copy)]
struct Element(usize);

impl SegTreeValue for Element {
    fn op(l: Self, r: Self) -> Self {
        Element(std::cmp::max(l.0, r.0))
    }

    fn e() -> Self {
        Element(usize::MIN)
    }
}

#[derive(Clone, Copy)]
struct Update(usize);

impl LazySegTreeUpdate<Element> for Update {
    fn apply(upd: Self, val: Element) -> Element {
        Element(val.0 + upd.0)
    }

    fn combine(upd: Self, other: Self) -> Self {
        Self(upd.0 + other.0)
    }

    fn id() -> Self {
        Self(0)
    }
}

#[test]
fn lazy_seg_tree_example() {
    let mut seg_tree = LazySegTree::new(&[Element(1), Element(2), Element(3)]);
    assert_get(&mut seg_tree, 0..3, 3);
    seg_tree.apply(0..=1, Update(4));
    assert_get(&mut seg_tree, 0..3, 6);
    assert_get(&mut seg_tree, 2..3, 3);
    assert_get(&mut seg_tree, 0..1, 5);
}

#[test]
fn lazy_seg_tree_rand_small() {
    run_rand_test(4, 1000, 42);
}

#[test]
fn lazy_seg_tree_rand_big() {
    run_rand_test(10, 1000, 7_40);
}

#[track_caller]
fn run_rand_test(n: usize, iters: usize, seed: usize) {
    let mut rand = Random::new(seed);
    let mut state = vec![0usize; n];
    let mut seg_tree = LazySegTree::with_size(n);
    for _ in 0..iters {
        let is_upd = rand.gen_range(0..10) == 0;
        let start = rand.gen_range(0..n);
        let end = rand.gen_range(start + 1..=n);
        if is_upd {
            let upd = rand.gen_range(1..10);
            for i in start..end {
                state[i] += upd;
            }
            seg_tree.apply(start..end, Update(upd));
        } else {
            let expected = (start..end).map(|i| state[i]).max().unwrap();
            assert_get(&mut seg_tree, start..end, expected);
        }
    }
}

#[track_caller]
fn assert_get(seg_tree: &mut LazySegTree<Element, Update>, rng: Range<usize>, expected: usize) {
    let actual = seg_tree.get(rng.clone()).0;
    assert_eq!(actual, expected, "get failed for range {rng:?}")
}
