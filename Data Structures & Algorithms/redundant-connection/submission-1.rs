struct UnionFind {
    pi: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let (mut pi, mut sz) = (vec![0; n], vec![1; n]);
        for i in 0..n {
            pi[i] = i;
        }
        Self {
            pi,
            sz,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        assert!(x < self.pi.len());
        if x != self.pi[x] {
            self.pi[x] = self.find(self.pi[x]);
        }
        self.pi[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        assert!(x < self.pi.len() && y < self.pi.len());
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        
        if self.sz[rx] < self.sz[ry] {
            self.pi[rx] = self.pi[ry];
            self.sz[ry] += self.sz[rx];
        } else {
            self.pi[ry] = self.pi[rx];
            self.sz[rx] += self.sz[ry];
        }
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(edges.len());
        for edge in edges {
            let x = (edge[0] - 1) as usize;
            let y = (edge[1] - 1) as usize;
            if uf.find(x) == uf.find(y) {
                return edge;
            } else {
                uf.union(x, y);
            }
        }
        vec![]
    }
}
