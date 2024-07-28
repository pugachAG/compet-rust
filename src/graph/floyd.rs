use std::ops::Add;

use super::weighted::WeightedGraph;

pub fn floyd<W>(g: &WeightedGraph<W>) -> Vec<Vec<Option<W>>>
where
    W: Copy + Ord + From<u8> + Add<W, Output = W>,
{
    let n = g.node_count();
    let mut ret = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| if i == j { Some(W::from(0u8)) } else { None })
                .collect::<Vec<Option<W>>>()
        })
        .collect::<Vec<_>>();
    for v in 0..n {
        for (u, w) in g.neighbours(v) {
            update_min(&mut ret[v][u], w);
        }
    }
    for k in 0..n {
        for v in 0..n {
            for u in 0..n {
                if let (Some(l1), Some(l2)) = (ret[v][k], ret[k][u]) {
                    update_min(&mut ret[v][u], l1 + l2);
                }
            }
        }
    }
    ret
}

fn update_min<W: Copy + Ord>(cur: &mut Option<W>, v: W) {
    if cur.map_or(true, |cur| v < cur) {
        *cur = Some(v);
    }
}
