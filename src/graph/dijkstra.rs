use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

use super::weighted::{NodeIndex, WeightedGraph};

pub fn dijkstra<W>(g: &WeightedGraph<W>, v0: NodeIndex) -> Vec<Option<W>>
where
    W: Copy + Ord + From<u8> + Add<W, Output = W>,
{
    let n = g.node_count();
    let mut dist = vec![None; n];
    let mut q = BinaryHeap::new();
    let dv0 = W::from(0u8);
    dist[v0] = Some(dv0);
    q.push(Reverse((dv0, v0)));
    while let Some(Reverse((dv, v))) = q.pop() {
        if dist[v] != Some(dv) {
            continue;
        }
        for (u, w) in g.neighbours(v) {
            let du = dv + w;
            if dist[u].map_or(true, |cur| du < cur) {
                dist[u] = Some(du);
                q.push(Reverse((du, u)));
            }
        }
    }
    dist
}
