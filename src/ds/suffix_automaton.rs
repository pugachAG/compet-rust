use std::{collections::HashMap, hash::Hash, marker::PhantomData};

type NodeIndex = usize;

const NO_NODE_INDEX: NodeIndex = usize::MAX;

pub trait AlphabetMap<C>: Clone + Default {
    fn get(&self, c: &C) -> Option<NodeIndex>;
    fn insert(&mut self, c: C, v: NodeIndex);
}

#[derive(Clone)]
pub struct ByteRangeMap<const O: u8, const L: usize>([usize; L]);

pub type LowerCaseMap = ByteRangeMap<b'a', 26>;

impl<const O: u8, const L: usize> Default for ByteRangeMap<O, L> {
    fn default() -> Self {
        Self([NO_NODE_INDEX; L])
    }
}

impl<const O: u8, const S: usize> AlphabetMap<u8> for ByteRangeMap<O, S> {
    fn get(&self, c: &u8) -> Option<NodeIndex> {
        let i = self.0[(c - O) as usize];
        if i == NO_NODE_INDEX {
            None
        } else {
            Some(i)
        }
    }

    fn insert(&mut self, c: u8, v: NodeIndex) {
        self.0[(c - O) as usize] = v;
    }
}

#[derive(Clone)]
pub struct AlphabetHashMap<C>(HashMap<C, NodeIndex>);

impl<C> Default for AlphabetHashMap<C> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<C: Hash + Eq + Clone> AlphabetMap<C> for AlphabetHashMap<C> {
    fn get(&self, c: &C) -> Option<NodeIndex> {
        self.0.get(c).cloned()
    }

    fn insert(&mut self, c: C, v: NodeIndex) {
        self.0.insert(c, v);
    }
}

struct Node<C, M: AlphabetMap<C>> {
    pub len: usize,
    pub link: Option<NodeIndex>,
    pub nxt: M,
    phantom: PhantomData<C>,
}

/// https://cp-algorithms.com/string/suffix-automaton.html
pub struct SuffixAutomaton<C, M: AlphabetMap<C>> {
    nodes: Vec<Node<C, M>>,
    last_i: NodeIndex,
}

pub type LowerCaseSuffixAutomaton = SuffixAutomaton<u8, LowerCaseMap>;
pub type HashMapSuffixAutomaton<C> = SuffixAutomaton<C, AlphabetHashMap<C>>;

impl<C: Copy, M: AlphabetMap<C>> SuffixAutomaton<C, M> {
    pub fn new() -> Self {
        let root = Node {
            len: 0,
            link: None,
            nxt: M::default(),
            phantom: PhantomData,
        };
        Self {
            nodes: vec![root],
            last_i: 0,
        }
    }

    pub fn contains(&self, seq: impl Iterator<Item = C>) -> bool {
        self.find(seq).is_some()
    }

    pub fn add_all(&mut self, seq: impl ExactSizeIterator<Item = C>) {
        self.nodes.reserve(2 * seq.len());
        for c in seq {
            self.add(c);
        }
    }

    pub fn add(&mut self, c: C) {
        let last_i = self.last_i;
        let cur_i = self.create_node(self.nodes[last_i].len + 1, 0, M::default());
        self.last_i = cur_i;
        let mut p_i = last_i;
        let q_i = loop {
            let p = &mut self.nodes[p_i];
            if let Some(q_i) = p.nxt.get(&c) {
                break q_i;
            }
            p.nxt.insert(c, cur_i);
            if let Some(node_i) = p.link {
                p_i = node_i;
            } else {
                return;
            };
        };
        let (p, q) = (&self.nodes[p_i], &self.nodes[q_i]);
        if q.len == p.len + 1 {
            self.nodes[cur_i].link = Some(q_i);
            return;
        }
        let clone_i = self.create_node(p.len + 1, q.link.unwrap(), q.nxt.clone());
        loop {
            let p = &mut self.nodes[p_i];
            if p.nxt.get(&c) == Some(q_i) {
                p.nxt.insert(c, clone_i);
            } else {
                break;
            }
            if let Some(node_i) = p.link {
                p_i = node_i;
            } else {
                break;
            };
        }
        self.nodes[cur_i].link = Some(clone_i);
        self.nodes[q_i].link = Some(clone_i);
    }

    fn find(&self, seq: impl Iterator<Item = C>) -> Option<&Node<C, M>> {
        let mut cur_i = 0;
        for c in seq {
            if let Some(node_i) = self.nodes[cur_i].nxt.get(&c) {
                cur_i = node_i;
            } else {
                return None;
            }
        }
        Some(&self.nodes[cur_i])
    }

    fn create_node(&mut self, len: usize, link: NodeIndex, nxt: M) -> NodeIndex {
        let i = self.nodes.len();
        self.nodes.push(Node {
            len,
            link: Some(link),
            nxt,
            phantom: PhantomData,
        });
        i
    }
}
