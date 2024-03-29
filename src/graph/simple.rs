use crate::utils::collections::def_vec;

pub type NodeIndex = usize;

#[derive(Debug)]
pub struct SimpleGraph {
    edges: Vec<Vec<NodeIndex>>,
}

impl SimpleGraph {
    pub fn new(n: usize) -> Self {
        Self { edges: def_vec(n) }
    }

    pub fn with_edges(n: usize, edges: &[(NodeIndex, NodeIndex)]) -> Self {
        let mut ret = Self::new(n);
        for &(v, u) in edges {
            ret.add_edge(v, u);
        }
        ret
    }

    pub fn node_count(&self) -> usize {
        self.edges.len()
    }

    pub fn neighbours<'a>(&'a self, v: NodeIndex) -> impl Iterator<Item = NodeIndex> + 'a {
        self.edges[v].iter().cloned()
    }

    pub fn add_node(&mut self) -> NodeIndex {
        let node_idx = self.node_count();
        self.edges.push(Vec::new());
        node_idx
    }

    pub fn add_edge(&mut self, v: NodeIndex, u: NodeIndex) {
        self.add_direct_edge(v, u);
        self.add_direct_edge(u, v);
    }

    pub fn add_direct_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.edges[from].push(to);
    }
}
