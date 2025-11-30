use std::{collections::HashMap, hash::Hash, marker::PhantomData};

pub trait AlphabetMap<C, V>: Clone + Default {
    fn get(&self, c: &C) -> Option<&V>;
    fn insert(&mut self, k: C, v: V);
}

#[derive(Clone)]
pub struct RangeMap<V, const O: u8, const L: usize>([Option<V>; L]);

pub type LowerCaseMap<V> = RangeMap<V, b'a', 26>;

impl<V, const O: u8, const L: usize> Default for RangeMap<V, O, L> {
    fn default() -> Self {
        Self(std::array::from_fn(|_| None))
    }
}

impl<V, const O: u8, const L: usize> RangeMap<V, O, L> {
    pub fn get(&self, k: u8) -> Option<&V> {
        let i = self.index(k);
        self.0[i].as_ref()
    }

    pub fn insert(&mut self, k: u8, v: V) -> Option<V> {
        let i = self.index(k);
        std::mem::replace(&mut self.0[i], Some(v))
    }

    #[inline]
    fn index(&self, k: u8) -> usize {
        (k - O) as usize
    }
}

impl<V: Clone, const O: u8, const S: usize> AlphabetMap<u8, V> for RangeMap<V, O, S> {
    fn get(&self, c: &u8) -> Option<&V> {
        self.get(*c)
    }

    fn insert(&mut self, k: u8, v: V) {
        self.insert(k, v);
    }
}

#[derive(Clone)]
pub struct AlphabetHashMap<C, V>(HashMap<C, V>);

impl<C, V> Default for AlphabetHashMap<C, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<C: Hash + Eq + Clone, V: Clone> AlphabetMap<C, V> for AlphabetHashMap<C, V> {
    fn get(&self, c: &C) -> Option<&V> {
        self.0.get(c)
    }

    fn insert(&mut self, k: C, v: V) {
        self.0.insert(k, v);
    }
}

type NodeIndex = usize;

struct Node<C, M: AlphabetMap<C, NodeIndex>> {
    pub len: usize,
    pub link: Option<NodeIndex>,
    pub nxt: M,
    phantom: PhantomData<C>,
}

/// https://cp-algorithms.com/string/suffix-automaton.html
pub struct SuffixAutomaton<C, M: AlphabetMap<C, NodeIndex>> {
    nodes: Vec<Node<C, M>>,
    last_i: NodeIndex,
}

pub type LowerCaseSuffixAutomaton = SuffixAutomaton<u8, LowerCaseMap<NodeIndex>>;
pub type HashMapSuffixAutomaton<C> = SuffixAutomaton<C, AlphabetHashMap<C, NodeIndex>>;

impl<C: Copy, M: AlphabetMap<C, NodeIndex>> SuffixAutomaton<C, M> {
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
            if let Some(&q_i) = p.nxt.get(&c) {
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
            if p.nxt.get(&c) == Some(&q_i) {
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
            if let Some(&node_i) = self.nodes[cur_i].nxt.get(&c) {
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
