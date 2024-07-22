use std::ops::Range;

use crate::ds::seg_tree::{SegTree, SegTreeValue};
use crate::ds::seg_tree_values::{seg_tree_value_max, seg_tree_value_sum};
use crate::plat::classic::includes::IntoVecExt;

#[test]
fn seg_tree_example() {
    #[derive(Copy, Clone)]
    struct SegTreeEl(i32);

    impl SegTreeValue for SegTreeEl {
        fn op(l: Self, r: Self) -> Self {
            Self(l.0 + r.0)
        }

        fn e() -> Self {
            Self(0)
        }
    }

    let mut st = SegTree::with_len(3);
    st.set(0, SegTreeEl(1));
    st.set(2, SegTreeEl(2));
    let SegTreeEl(sum) = st.get(0..=2);
    assert_eq!(sum, 3);
}

#[test]
fn seg_tree_sum() {
    seg_tree_value_sum!(Element, i32);
    const DATA: [i32; 18] = [1, 10, 5, 6, -5, 4, 3, 13, 3, 4, 8, 7, 9, 10, 8, 9, 9, 1];
    let n = DATA.len();
    for start in 0..n {
        for end in start..=n {
            let mut cur = DATA[start..end].to_vec();
            let mut st = SegTree::new(&cur.iter().map(|x| Element(*x)).into_vec());
            for i in 0..cur.len() {
                let v = (i + end - start) as i32;
                cur[i] = v;
                st.set(i, Element(v));
                for l in 0..cur.len() {
                    for r in l..=cur.len() {
                        let actual = st.get(l..r).0;
                        let expected = cur[l..r].iter().sum();
                        assert_eq!(actual, expected, "rng: {:?}, a = {cur:?}", l..r);
                    }
                }
            }
        }
    }
}

#[test]
fn seg_tree_max_prefix() {
    const DATA: [i32; 18] = [1, 10, 5, 6, -5, 4, 3, 13, 3, 4, 8, 7, 9, 10, 8, 9, 9, 1];
    let (mn, mx) = (*DATA.iter().min().unwrap(), *DATA.iter().max().unwrap());
    let n = DATA.len();
    for l in 0..n {
        for r in l..=n {
            for v in mn..=mx {
                check_seg_tree_max_prefix(&DATA, l..r, v)
            }
        }
    }
}

#[track_caller]
fn check_seg_tree_max_prefix(a: &[i32], rng: Range<usize>, v: i32) {
    seg_tree_value_max!(Element, i32);
    let st = SegTree::new(&a.iter().map(|x| Element(*x)).into_vec());
    let expected = {
        let mut ret = None;
        let mut best = i32::MIN;
        for i in rng.clone() {
            best = std::cmp::max(best, a[i]);
            if best < v {
                ret = Some(i);
            } else {
                break;
            }
        }
        ret
    };
    let actual = st.max_prefix(rng.clone(), |el| el.0 < v);
    assert_eq!(
        actual,
        expected,
        "rng: {:?} ({:?}), v: {v}",
        rng.clone(),
        &a[rng]
    );
}
