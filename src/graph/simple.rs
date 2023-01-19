use crate::utils::collections::def_vec;

pub type NodeIndex = usize;

pub struct SimpleGraph {
    edges: Vec<Vec<NodeIndex>>,
}

impl SimpleGraph {
    pub fn new(n: usize) -> Self {
        Self { edges: def_vec(n) }
    }

    pub fn node_count(&self) -> usize {
        self.edges.len()
    }

    pub fn neighbours(&self, v: NodeIndex) -> &[NodeIndex] {
        &self.edges[v]
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
