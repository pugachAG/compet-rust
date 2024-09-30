pub struct Dsu {
    p: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
        }
    }

    pub fn parent(&mut self, v: usize) -> usize {
        if self.p[v] != v {
            self.p[v] = self.parent(self.p[v]);
        }
        self.p[v]
    }

    pub fn join(&mut self, v: usize, u: usize) -> bool {
        let pv = self.parent(v);
        let pu = self.parent(u);
        if pv != pu {
            self.p[pv] = pu;
        }
        pv != pu
    }
}
