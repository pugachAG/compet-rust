use crate::graph::simple::{NodeIndex, SimpleGraph};

pub fn undirected_graph(n: usize, edges: &[(NodeIndex, NodeIndex)]) -> SimpleGraph {
    let mut g = SimpleGraph::new(n);
    for &(v, u) in edges {
        g.add_edge(v, u);
    }
    g
}
