use crate::utils::collections::vec_vec;

pub struct SimpleGraph {
    n: usize,
    edges: Vec<Vec<usize>>
}

impl SimpleGraph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: vec_vec(n)
        }
    }

    pub fn neighbours(&self, v: usize) -> &[usize] {
        &self.edges[v]
    }

    pub fn add_edge(&mut self, v: usize, u: usize) {
        self.add_direct_edge(v, u);
        self.add_direct_edge(u, v);
    }

    pub fn add_direct_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.n && to < self.n);
        self.edges[from].push(to);
    }
}