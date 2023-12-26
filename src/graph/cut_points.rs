use super::simple::{NodeIndex, SimpleGraph};

pub type ConCompCount = usize;

#[derive(Debug, Eq, PartialEq)]
pub struct CutConComps {
    /// Number of connected componnts in the original graph
    pub cc: ConCompCount,
    /// Number of connected components after removal of each vertex
    pub cut_cc: Vec<ConCompCount>,
}

impl CutConComps {
    pub fn cut_vertices<'a>(&'a self) -> impl Iterator<Item = NodeIndex> + 'a {
        self.cut_cc
            .iter()
            .enumerate()
            .filter(|p| *p.1 > self.cc)
            .map(|p| p.0)
    }
}

/// https://cp-algorithms.com/graph/cutpoints.html
/// The graph expected to be undirected.
pub fn cut_connected_components(g: &SimpleGraph) -> CutConComps {
    let n = g.node_count();
    let mut state = State::new(n);
    let mut cc = 0;
    for v in 0..n {
        if !state.visited(v) {
            cc += 1;
            state.dfs(v, None, g);
        }
    }
    CutConComps {
        cc,
        cut_cc: state
            .cc_delta
            .into_iter()
            .map(|d| cc.checked_add_signed(d).unwrap())
            .collect(),
    }
}

type Time = usize;
type ConCompDelta = isize;

struct State {
    tin: Vec<Time>,
    low: Vec<Time>,
    nxt_time: usize,
    cc_delta: Vec<ConCompDelta>,
}

impl State {
    const NOT_VISITED_TIME: Time = 0;

    fn new(n: usize) -> Self {
        Self {
            tin: vec![Self::NOT_VISITED_TIME; n],
            low: vec![Self::NOT_VISITED_TIME; n],
            nxt_time: Self::NOT_VISITED_TIME + 1,
            cc_delta: vec![0; n],
        }
    }

    fn visited(&self, v: usize) -> bool {
        self.tin[v] != Self::NOT_VISITED_TIME
    }

    fn dfs(&mut self, v: usize, p: Option<usize>, g: &SimpleGraph) {
        let tin = self.nxt_time;
        self.nxt_time += 1;
        self.tin[v] = tin;
        let mut low = tin;
        let mut isolated_subtrees = 0;
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
                if self.low[u] >= tin {
                    isolated_subtrees += 1;
                }
            }
        }
        self.low[v] = low;
        self.cc_delta[v] = if p.is_some() {
            isolated_subtrees
        } else {
            children - 1
        };
    }
}
