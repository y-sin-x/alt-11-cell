#[derive(Clone)]
pub struct Permutation {
    pub perm: Vec<usize>,
    pub deg: usize,
}

impl Permutation {
    pub fn new(perm: Vec<usize>) -> Permutation {
        let deg = perm.len();
        Self { perm, deg }
    }

    pub fn identity(degree: usize) -> Permutation {
        Self::new((0..degree).collect())
    }

    pub fn inverse(&self) -> Self {
        let mut inv = Self {
            perm: vec![0; self.deg],
            deg: self.deg,
        };
        for i in 0..self.deg {
            inv.perm[self.perm[i]] = i;
        }
        inv
    }

    pub fn product(&self, other: &Self) -> Self {
        let mut prod = vec![0; self.deg];
        for i in 0..self.deg {
            prod[i] = other.perm[self.perm[i]];
        }
        Self::new(prod)
    }

    pub fn exp(&self, n: usize) -> Self {
        let mut e = Permutation::identity(self.deg);
        for i in 0..self.deg {
            for _ in 0..n {
                e.perm[i] = self.perm[e.perm[i]];
            }
        }
        e
    }

    pub fn permute(&self, num: usize) -> usize {
        self.perm[num]
    }
}
