use super::simple::{NodeIndex, SimpleGraph};

/// https://cp-algorithms.com/graph/cutpoints.html
/// The graph expected to be undirected.
pub fn cut_points(g: &SimpleGraph) -> Vec<NodeIndex> {
    let n = g.node_count();
    let mut state = State::new(n);
    for v in 0..n {
        if !state.visited(v) {
            state.dfs(v, None, g);
        }
    }
    state.ans
}

struct State {
    tin: Vec<usize>,
    low: Vec<usize>,
    timer: usize,
    ans: Vec<usize>,
}

impl State {
    const NOT_VISITED: usize = 0;

    fn new(n: usize) -> Self {
        Self {
            tin: vec![Self::NOT_VISITED; n],
            low: vec![Self::NOT_VISITED; n],
            timer: Self::NOT_VISITED + 1,
            ans: vec![],
        }
    }

    fn visited(&self, v: usize) -> bool {
        self.tin[v] != Self::NOT_VISITED
    }

    fn dfs(&mut self, v: usize, p: Option<usize>, g: &SimpleGraph) {
        let tin = self.timer;
        self.timer += 1;
        self.tin[v] = tin;
        let mut low = tin;
        let mut has_isolated_subtree = false;
        let mut children = 0;
        for u in g.neighbours(v) {
            if Some(u) == p {
                continue;
            }
            if self.visited(u) {
                low = low.min(self.tin[u]);
            } else {
                children += 1;
                self.dfs(u, Some(v), g);
                low = low.min(self.low[u]);
                has_isolated_subtree |= self.low[u] >= tin;
            }
        }
        let is_cut_point = if p.is_some() {
            has_isolated_subtree
        } else {
            children > 1
        };
        if is_cut_point {
            self.ans.push(v);
        }
        self.low[v] = low;
    }
}
