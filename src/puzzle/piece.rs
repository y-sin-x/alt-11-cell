use crate::puzzle::perm::Permutation;

#[derive(Clone)]
pub struct Piece {
    pub sig: Vec<u8>,
    pub att: Permutation,
}

impl Piece {
    pub fn new(sig: Vec<u8>) -> Self {
        let deg = sig.len();
        Self {
            sig,
            att: Permutation::identity(deg),
        }
    }

    pub fn degree(&self) -> usize {
        self.att.deg
    }

    pub fn grip_state(&self, grip: usize) -> u8 {
        self.sig[grip]
    }

    pub fn rotate(&self, rot: &Permutation) -> Self {
        let mut new_sig = vec![0; self.degree()];
        for i in 0..self.degree() {
            new_sig[rot.permute(i)] = self.sig[i];
        }
        Self {
            att: self.att.product(&rot),
            sig: new_sig,
        }
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..self.degree() {
            if self.sig[i] != self.sig[self.att.permute(i)] {
                return false;
            }
        }
        true
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.sig == other.sig
    }
}
