use std::collections::VecDeque;

use super::simple::{NodeIndex, SimpleGraph};

pub fn connected_components(g: &SimpleGraph) -> Vec<Vec<NodeIndex>> {
    let mut was = vec![false; g.node_count()];
    let mut ret = Vec::new();
    for v0 in 0..g.node_count() {
        if was[v0] {
            continue;
        }
        was[v0] = true;
        let mut cur = vec![v0];
        let mut i = 0;
        while let Some(&v) = cur.get(i) {
            for u in g.neighbours(v) {
                if !was[u] {
                    was[u] = true;
                    cur.push(u);
                }
            }
            i += 1;
        }
        ret.push(cur);
    }
    ret
}

/// Returns mapping from node index to connected component index
pub fn connected_components_index(g: &SimpleGraph) -> Vec<usize> {
    let n = g.node_count();
    let not_visited = n;
    let mut vis = vec![not_visited; n];
    let mut cur = 0;
    let mut q = VecDeque::new();
    for v0 in 0..g.node_count() {
        if vis[v0] != not_visited {
            continue;
        }
        vis[v0] = cur;
        q.push_back(v0);
        while let Some(v) = q.pop_front() {
            for u in g.neighbours(v) {
                if vis[u] == not_visited {
                    vis[u] = cur;
                    q.push_back(u);
                }
            }
        }
        cur += 1;
    }
    vis
}
