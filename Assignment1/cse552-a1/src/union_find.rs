use std::cell::Cell;

/// Disjoint-set union (union-find) with path compression and union by rank.
#[derive(Debug, Clone, Default)]
pub struct UnionFind {
    parent: Vec<Cell<usize>>,
    rank: Vec<Cell<usize>>,
}

impl UnionFind {
    /// Creates an empty union-find structure.
    pub fn new() -> Self {
        Self {
            parent: Vec::new(),
            rank: Vec::new(),
        }
    }

    /// Adds a new singleton set and returns its element id.
    pub fn make_set(&mut self) -> usize {
        let id = self.parent.len();
        self.parent.push(Cell::new(id));
        self.rank.push(Cell::new(0));
        id
    }

    /// Returns the representative of `id`, compressing traversed paths.
    pub fn find(&self, id: usize) -> usize {
        let mut v = id;
        while self.parent[v].get() != v {
            v = self.parent[v].get();
        }
        let root = v;

        let mut v = id;
        while self.parent[v].get() != v {
            let p = self.parent[v].get();
            self.parent[v].set(root);
            v = p;
        }

        root
    }

    /// Merges the sets containing `x` and `y`.
    pub fn union(&self, x: usize, y: usize) {
        let xr = self.find(x);
        let yr = self.find(y);

        if xr == yr {
            return;
        }

        let xrank = self.rank[xr].get();
        let yrank = self.rank[yr].get();

        if xrank < yrank {
            self.parent[xr].set(yr);
        } else if xrank > yrank {
            self.parent[yr].set(xr);
        } else {
            self.parent[yr].set(xr);
            self.rank[xr].set(xrank + 1);
        }
    }
}
