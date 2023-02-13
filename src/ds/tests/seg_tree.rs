use crate::ds::seg_tree::SegTree;

#[test]
pub fn seg_tree_sum() {
    let a = vec![1, 10, 5, 6, -5, 4, 3, 13, 3, 4, 8, 7, 9, 10, 8, 9, 9, 1];
    let n = a.len();
    for start in 0..n {
        for end in start..=n {
            let mut cur = a[start..end].to_vec();
            let mut st = SegTree::new(&cur, |x, y| *x + *y);
            for i in 0..cur.len() {
                let v = (i + end - start) as i32;
                cur[i] = v;
                st.set(i, v);
                for l in 0..cur.len() {
                    for r in l..cur.len() {
                        let actual = st.get(l, r);
                        let expected = cur[l..=r].iter().sum();
                        assert_eq!(actual, expected, "l = {l}, r = {r}, a = {cur:?}");
                    }
                }
            }
        }
    }
}
