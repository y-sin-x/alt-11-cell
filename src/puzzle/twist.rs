use crate::puzzle::perm::Permutation;

#[derive(Clone)]
pub struct Twist {
    pub grip: usize,
    pub rot: Permutation,
}

impl Twist {
    pub fn inverse(&self) -> Twist {
        Twist {
            grip: self.grip,
            rot: self.rot.inverse(),
        }
    }
}
