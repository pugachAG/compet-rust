use crate::graph::tests::utils::undirected_graph;
use crate::utils::rec_fn::{Callable2, RecFn2};

#[test]
fn rec_fn_tree_dfs() {
    let n = 4;
    let g = undirected_graph(n, &[(0, 1), (1, 3), (0, 2)]);
    let mut timer = 0;
    let mut tin = vec![0; n];
    RecFn2::new(|dfs, v: usize, p: usize| {
        tin[v] = timer;
        timer += 1;
        for u in g.neighbours(v) {
            if u != p {
                dfs.call(u, v);
            }
        }
    })
    .call(0, 0);
    assert_eq!(tin, vec![0, 1, 3, 2]);
}
