use rand::RngExt;

use crate::puzzle::{perm::Permutation, piece::Piece, setup11c, twist::Twist};

pub struct PuzzleState {
    pub degree: usize,
    pub pieces: Vec<Piece>,
    pub twist_stack: Vec<Twist>,
    pub is_solved: bool,
}

impl PuzzleState {
    pub fn generate(mut base_pieces: Vec<Piece>, generators: &Vec<Permutation>) -> Self {
        let mut gen_pieces = Vec::new();
        while !base_pieces.is_empty() {
            'generator_loop: for g in generators {
                let mut new = base_pieces[0].clone().rotate(g);
                for piece in &gen_pieces {
                    if new.overlaps(piece) {
                        continue 'generator_loop;
                    }
                }
                new.att = Permutation::identity(new.degree());
                base_pieces.push(new);
            }
            gen_pieces.push(base_pieces.swap_remove(0));
        }
        Self {
            degree: generators[0].deg,
            pieces: gen_pieces,
            twist_stack: Vec::new(),
            is_solved: true,
        }
    }

    pub fn twist(&mut self, twist: &Twist) {
        let mut new_pieces = Vec::new();
        for piece in &self.pieces {
            if piece.grip_state(twist.grip) == 1 {
                new_pieces.push(piece.rotate(&twist.rot));
            } else {
                new_pieces.push(piece.clone());
            }
        }
        self.pieces = new_pieces;
        self.is_solved = self.check_solved();
    }

    pub fn twist_move(&mut self, twist: &Twist) {
        self.twist(twist);
        self.twist_stack.push(twist.clone());
    }

    pub fn undo(&mut self) {
        if let Some(t) = self.twist_stack.pop() {
            self.twist(&t.inverse());
        }
    }

    pub fn scramble(&mut self, n: u32) {
        let mut rng = rand::rng();

        for _ in 0..n {
            let g = rng.random_range(0..self.degree);
            let f = rng.random_range(1..self.degree);

            let recenter = setup11c::cell_recenter(g);
            let face_recenter = setup11c::face_recenter(f);

            self.twist(&Twist {
                grip: g,
                rot: recenter
                    .product(&face_recenter)
                    .product(&setup11c::face_rot(rng.random_bool(0.5)))
                    .product(&face_recenter.inverse())
                    .product(&recenter.inverse()),
            })
        }
    }

    pub fn reset(&mut self) {
        let mut new_pieces = Vec::new();
        for piece in &self.pieces {
            new_pieces.push(piece.rotate(&piece.att.inverse()));
        }
        self.pieces = new_pieces;
    }

    pub fn check_solved(&mut self) -> bool {
        for piece in &self.pieces {
            if !piece.is_solved() {
                return false;
            }
        }
        true
    }
}
