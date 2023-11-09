use crate::ds::seg_tree::{SegTree, SegTreeMonoid};
use crate::plat::classic::includes::IntoVecExt;

#[test]
pub fn seg_tree_example() {
    #[derive(Copy, Clone)]
    struct SegTreeEl(i32);

    impl SegTreeMonoid for SegTreeEl {
        fn op(l: Self, r: Self) -> Self {
            Self(l.0 + r.0)
        }

        fn e() -> Self {
            Self(0)
        }
    }

    let mut st = SegTree::with_size(3);
    st.set(0, SegTreeEl(1));
    st.set(2, SegTreeEl(2));
    let SegTreeEl(sum )= st.get(0..=2);
    assert_eq!(sum, 3);
}

#[test]
pub fn seg_tree_sum() {
    #[derive(Copy, Clone)]
    struct SegTreeEl(i32);
    impl SegTreeMonoid for SegTreeEl {
        fn op(l: Self, r: Self) -> Self {
            Self(l.0 + r.0)
        }
        fn e() -> Self {
            Self(0)
        }
    }
    let a = vec![1, 10, 5, 6, -5, 4, 3, 13, 3, 4, 8, 7, 9, 10, 8, 9, 9, 1];
    let n = a.len();
    for start in 0..n {
        for end in start..=n {
            let mut cur = a[start..end].to_vec();
            let mut st = SegTree::new(&cur.iter().map(|x| SegTreeEl(*x)).into_vec());
            for i in 0..cur.len() {
                let v = (i + end - start) as i32;
                cur[i] = v;
                st.set(i, SegTreeEl(v));
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
