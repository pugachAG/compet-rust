use crate::utils::collections::def_vec;

pub type NodeIndex = usize;

#[derive(Debug)]
pub struct WeightedGraph<W> {
    edges: Vec<Vec<(NodeIndex, W)>>,
}

impl<W: Copy> WeightedGraph<W> {
    pub fn new(n: usize) -> Self {
        Self { edges: def_vec(n) }
    }

    pub fn with_edges(n: usize, edges: &[(NodeIndex, NodeIndex, W)]) -> Self {
        Self::with_edges_impl(n, edges, false)
    }

    pub fn with_direct_edges(n: usize, edges: &[(NodeIndex, NodeIndex, W)]) -> Self {
        Self::with_edges_impl(n, edges, true)
    }

    pub fn node_count(&self) -> usize {
        self.edges.len()
    }

    pub fn neighbour_count(&self, v: NodeIndex) -> usize {
        self.edges[v].len()
    }

    pub fn neighbours<'a>(&'a self, v: NodeIndex) -> impl Iterator<Item = (NodeIndex, W)> + 'a {
        self.edges[v].iter().cloned()
    }

    pub fn add_node(&mut self) -> NodeIndex {
        let node_idx = self.node_count();
        self.edges.push(Vec::new());
        node_idx
    }

    pub fn add_edge(&mut self, v: NodeIndex, u: NodeIndex, w: W) {
        self.add_direct_edge(v, u, w);
        self.add_direct_edge(u, v, w);
    }

    pub fn add_direct_edge(&mut self, from: NodeIndex, to: NodeIndex, w: W) {
        self.edges[from].push((to, w));
    }

    fn with_edges_impl(n: usize, edges: &[(NodeIndex, NodeIndex, W)], direct: bool) -> Self {
        let mut ret = Self::new(n);
        for &(v, u, w) in edges {
            if direct {
                ret.add_direct_edge(v, u, w)
            } else {
                ret.add_edge(v, u, w);
            }
        }
        ret
    }
}
