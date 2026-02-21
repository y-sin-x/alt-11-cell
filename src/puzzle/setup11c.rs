use std::f32::consts::PI;

use eframe::egui::{Color32, Vec2};

use crate::puzzle::{
    perm::Permutation,
    piece::Piece,
    view::{Face, Substicker},
};

pub fn base_pieces() -> Vec<Piece> {
    vec![
        Piece::new(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        Piece::new(vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        Piece::new(vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
        Piece::new(vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]),
    ]
}

pub fn generators() -> Vec<Permutation> {
    vec![
        Permutation::new(vec![10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        Permutation::new(vec![0, 2, 3, 4, 5, 1, 7, 8, 9, 10, 6]),
        Permutation::new(vec![0, 6, 8, 3, 4, 9, 1, 10, 2, 5, 7]),
    ]
}

pub fn cell_recenter(n: usize) -> Permutation {
    match n {
        0 => Permutation::identity(11),
        1 => Permutation::new(vec![1, 0, 5, 3, 4, 2, 6, 8, 7, 10, 9]),
        2 => Permutation::new(vec![2, 3, 0, 1, 4, 5, 10, 7, 9, 8, 6]),
        3 => Permutation::new(vec![3, 1, 4, 0, 2, 5, 7, 6, 8, 10, 9]),
        4 => Permutation::new(vec![4, 1, 2, 5, 0, 3, 10, 8, 7, 9, 6]),
        5 => Permutation::new(vec![5, 4, 2, 3, 1, 0, 7, 6, 9, 8, 10]),
        6 => Permutation::new(vec![1, 6, 10, 4, 3, 7, 0, 8, 2, 5, 9]),
        7 => Permutation::new(vec![2, 8, 7, 6, 5, 4, 10, 0, 9, 3, 1]),
        8 => Permutation::new(vec![3, 5, 9, 8, 7, 1, 2, 6, 0, 10, 4]),
        9 => Permutation::new(vec![4, 2, 1, 10, 9, 8, 5, 3, 7, 0, 6]),
        10 => Permutation::new(vec![5, 9, 3, 2, 6, 10, 7, 1, 4, 8, 0]),
        _ => Permutation::identity(11),
    }
}

pub fn face_recenter(n: usize) -> Permutation {
    match n {
        1 => Permutation::identity(11),
        2 => Permutation::new(vec![0, 5, 1, 2, 3, 4, 10, 6, 7, 8, 9]),
        3 => Permutation::new(vec![0, 4, 5, 1, 2, 3, 9, 10, 6, 7, 8]),
        4 => Permutation::new(vec![0, 3, 4, 5, 1, 2, 8, 9, 10, 6, 7]),
        5 => Permutation::new(vec![0, 2, 3, 4, 5, 1, 7, 8, 9, 10, 6]),
        6 => Permutation::new(vec![0, 6, 8, 3, 4, 9, 1, 10, 2, 5, 7]),
        7 => Permutation::new(vec![0, 9, 6, 8, 3, 4, 7, 1, 10, 2, 5]),
        8 => Permutation::new(vec![0, 4, 9, 6, 8, 3, 5, 7, 1, 10, 2]),
        9 => Permutation::new(vec![0, 3, 4, 9, 6, 8, 2, 5, 7, 1, 10]),
        10 => Permutation::new(vec![0, 8, 3, 4, 9, 6, 10, 2, 5, 7, 1]),
        _ => Permutation::identity(11),
    }
}

pub fn face_rot(ccw: bool) -> Permutation {
    Permutation::new(if ccw {
        vec![0, 1, 5, 10, 8, 6, 2, 4, 7, 3, 9]
    } else {
        vec![0, 1, 6, 9, 7, 2, 5, 8, 4, 10, 3]
    })
}

pub fn cell_positions() -> Vec<Vec2> {
    let d = 2.3;
    vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, d),
        Vec2::new(d * f32::sin(0.4 * PI), d * f32::cos(0.4 * PI)),
        Vec2::new(d * f32::sin(0.8 * PI), d * f32::cos(0.8 * PI)),
        Vec2::new(-d * f32::sin(0.8 * PI), d * f32::cos(0.8 * PI)),
        Vec2::new(-d * f32::sin(0.4 * PI), d * f32::cos(0.4 * PI)),
        Vec2::new(0.0, 2.0 * d * f32::cos(0.8 * PI)),
        Vec2::new(
            -d * (f32::sin(0.4 * PI) + f32::sin(0.8 * PI)),
            d * (f32::cos(0.4 * PI) + f32::cos(0.8 * PI)),
        ),
        Vec2::new(-d * f32::sin(0.4 * PI), d * (1.0 + f32::cos(0.4 * PI))),
        Vec2::new(d * f32::sin(0.4 * PI), d * (1.0 + f32::cos(0.4 * PI))),
        Vec2::new(
            d * (f32::sin(0.4 * PI) + f32::sin(0.8 * PI)),
            d * (f32::cos(0.4 * PI) + f32::cos(0.8 * PI)),
        ),
    ]
}

pub fn cell_scales() -> Vec<f32> {
    vec![1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
}

pub fn faces() -> Vec<Face> {
    let d1 = 0.7;
    let d2 = 1.0;
    let v = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, -d1),
        Vec2::new(-d1 * f32::sin(0.4 * PI), -d1 * f32::cos(0.4 * PI)),
        Vec2::new(-d1 * f32::sin(0.8 * PI), -d1 * f32::cos(0.8 * PI)),
        Vec2::new(d1 * f32::sin(0.8 * PI), -d1 * f32::cos(0.8 * PI)),
        Vec2::new(d1 * f32::sin(0.4 * PI), -d1 * f32::cos(0.4 * PI)),
        Vec2::new(0.0, d2),
        Vec2::new(d2 * f32::sin(0.4 * PI), d2 * f32::cos(0.4 * PI)),
        Vec2::new(d2 * f32::sin(0.8 * PI), d2 * f32::cos(0.8 * PI)),
        Vec2::new(-d2 * f32::sin(0.8 * PI), d2 * f32::cos(0.8 * PI)),
        Vec2::new(-d2 * f32::sin(0.4 * PI), d2 * f32::cos(0.4 * PI)),
    ];

    vec![
        Face::new(vec![v[0], v[3], v[4]]),
        Face::new(vec![v[0], v[4], v[5]]),
        Face::new(vec![v[0], v[5], v[1]]),
        Face::new(vec![v[0], v[1], v[2]]),
        Face::new(vec![v[0], v[2], v[3]]),
        Face::new(vec![v[6], v[4], v[3]]),
        Face::new(vec![v[7], v[5], v[4]]),
        Face::new(vec![v[8], v[1], v[5]]),
        Face::new(vec![v[9], v[2], v[1]]),
        Face::new(vec![v[10], v[3], v[2]]),
    ]
}

pub fn colors(grip: usize) -> Color32 {
    match grip {
        0 => Color32::from_rgb(127, 127, 127),
        1 => Color32::WHITE,
        2 => Color32::GREEN,
        3 => Color32::YELLOW,
        4 => Color32::BLUE,
        5 => Color32::RED,
        6 => Color32::from_rgb(0, 191, 255),
        7 => Color32::from_rgb(255, 127, 255),
        8 => Color32::DARK_GREEN,
        9 => Color32::PURPLE,
        10 => Color32::from_rgb(255, 127, 0),
        _ => Color32::BLACK,
    }
}

pub fn substicker(face_piece: Piece, v: Vec<Vec2>) -> Substicker {
    match face_piece.sig[..] {
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0] => Substicker::Ridge(v),
        [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0] => Substicker::Edge(v[1], v[2], v[0], v[1]),
        [1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0] => Substicker::Edge(v[2], v[0], v[1], v[2]),
        [1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0] => Substicker::Edge(v[0], v[1], v[2], v[0]),
        [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0] => Substicker::Vertex(v[2], v[0], v[1]),
        [1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1] => Substicker::Vertex(v[0], v[1], v[2]),
        [1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0] => Substicker::Vertex(v[1], v[2], v[0]),
        _ => Substicker::None,
    }
}
