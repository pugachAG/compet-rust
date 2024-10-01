use std::collections::VecDeque;
use std::ops::Add;

use super::weighted::{NodeIndex, WeightedGraph};

#[derive(Clone, Copy)]
pub enum NodeDist<W> {
    Value(W),
    Inf,
    NegInf,
}

pub fn bellman_ford<W>(g: &WeightedGraph<W>, source: NodeIndex) -> Vec<NodeDist<W>>
where
    W: Copy + Ord + From<u8> + Add<W, Output = W>,
{
    let n = g.node_count();
    let edges = (0..n)
        .flat_map(|v| g.neighbours(v).map(move |(u, w)| (v, u, w)))
        .collect::<Vec<_>>();
    let mut dist = (0..n)
        .map(|v| {
            if v == source {
                NodeDist::Value(W::from(0u8))
            } else {
                NodeDist::Inf
            }
        })
        .collect::<Vec<_>>();
    let mut q = VecDeque::new();
    for iteration in 0..n {
        for &(v, u, w) in &edges {
            if let NodeDist::Value(dist_v) = dist[v] {
                let is_updated = update_dist(&mut dist[u], dist_v + w);
                if iteration == n - 1 && is_updated {
                    dist[u] = NodeDist::NegInf;
                    q.push_back(u);
                }
            }
        }
    }
    while let Some(v) = q.pop_front() {
        for (u, _) in g.neighbours(v) {
            if !matches!(dist[u], NodeDist::NegInf) {
                dist[u] = NodeDist::NegInf;
                q.push_back(u);
            }
        }
    }
    dist
}

fn update_dist<W>(d: &mut NodeDist<W>, w: W) -> bool
where
    W: Copy + Ord,
{
    let upd = match d {
        NodeDist::Value(cur) => (w < *cur).then_some(w),
        NodeDist::Inf => Some(w),
        NodeDist::NegInf => None,
    };
    if let Some(upd) = upd {
        *d = NodeDist::Value(upd);
        true
    } else {
        false
    }
}
