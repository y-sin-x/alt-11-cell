use eframe::{
    egui::{Color32, Pos2, Ui, Vec2},
    epaint::{PathShape, PathStroke},
};

use crate::puzzle::{setup11c, state::PuzzleState, twist::Twist};

pub struct Face {
    pub vertices: Vec<Vec2>,
    // pub center: Vec2,
}

impl Face {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        Self { vertices }
    }

    pub fn contains_pos(&self, pos: Vec2) -> bool {
        let n = self.vertices.len();
        for i in 0..n {
            let v1 = self.vertices[(i + 1) % n] - self.vertices[i];
            let v2 = pos - self.vertices[i];
            if v1.x * v2.y - v1.y * v2.x > 0.0 {
                return false;
            }
        }
        true
    }
}

pub enum Substicker {
    Ridge(Vec<Vec2>),
    Edge(Vec2, Vec2, Vec2, Vec2),
    Vertex(Vec2, Vec2, Vec2),
    None,
}

pub struct PuzzleView {
    pub state: PuzzleState,
    pub was_scrambled: bool,
    pub cell_pos: Vec<Vec2>,
    pub cell_scale: Vec<f32>,
    pub faces: Vec<Face>,
    pub alt_view: bool,
    pub edge_size: f32,
    pub gap_size: f32,
    pub scale: f32,
    pub offset: Pos2,
}

impl PuzzleView {
    pub fn show_puzzle(&mut self, ui: &mut Ui) {
        for piece in &self.state.pieces {
            for cell in 0..piece.degree() {
                let centered_piece = piece.rotate(&setup11c::cell_recenter(cell));
                for face in 1..piece.degree() {
                    if centered_piece.grip_state(face) == 1 {
                        let face_piece = centered_piece.rotate(&setup11c::face_recenter(face));
                        let face_grip = setup11c::cell_recenter(cell).inverse().permute(face);
                        let v = self.faces[face - 1].vertices.clone();
                        let inv_att = piece.att.inverse().clone();
                        let color = if self.alt_view {
                            setup11c::colors(inv_att.permute(face_grip))
                        } else {
                            setup11c::colors(inv_att.permute(cell))
                        };

                        match setup11c::substicker(face_piece, v) {
                            Substicker::Ridge(v) => self.draw_ridge(v, cell, color, ui),
                            Substicker::Edge(a1, a0, b0, b1) => {
                                self.draw_edge(a1, a0, b0, b1, cell, color, ui)
                            }
                            Substicker::Vertex(a, b, c) => {
                                self.draw_vertex(a, b, c, cell, color, ui)
                            }
                            Substicker::None => (),
                        }
                    }
                }
            }
        }
    }

    pub fn pointer_twist(&mut self, pos: Pos2, ccw: bool) {
        for cell in 0..self.state.degree {
            if pos.distance(self.calc_pos(self.cell_pos[cell]))
                < self.scale * f32::abs(self.cell_scale[cell])
            {
                let rel_pos = ((pos - self.offset) / self.scale - self.cell_pos[cell])
                    / self.cell_scale[cell];
                let recenter = setup11c::cell_recenter(cell);

                for face in 1..self.state.degree {
                    if self.faces[face - 1].contains_pos(rel_pos) {
                        let face_recenter = setup11c::face_recenter(face);

                        self.state.twist_move(&Twist {
                            grip: cell,
                            rot: recenter
                                .product(&face_recenter)
                                .product(&setup11c::face_rot(ccw))
                                .product(&face_recenter.inverse())
                                .product(&recenter.inverse()),
                        });

                        break;
                    }
                }

                break;
            }
        }
    }

    pub fn draw_ridge(&self, v: Vec<Vec2>, cell: usize, color: Color32, ui: &mut Ui) {
        let n = v.len();

        let mut points = Vec::new();
        for i in 0..n {
            points.push(self.calc_pos(
                self.cell_pos[cell]
                    + (v[i]
                        + (self.edge_size + self.gap_size)
                            * (v[(i + 1) % n] + v[(i + 2) % n] - 2.0 * v[i]))
                        * self.cell_scale[cell],
            ))
        }

        ui.painter()
            .add(PathShape::convex_polygon(points, color, PathStroke::NONE));
    }

    pub fn draw_edge(
        &self,
        a1: Vec2,
        a0: Vec2,
        b0: Vec2,
        b1: Vec2,
        cell: usize,
        color: Color32,
        ui: &mut Ui,
    ) {
        ui.painter().add(PathShape::convex_polygon(
            vec![
                self.calc_pos(
                    self.cell_pos[cell]
                        + (a0 + (self.edge_size + self.gap_size) * (b0 - a0))
                            * self.cell_scale[cell],
                ),
                self.calc_pos(
                    self.cell_pos[cell]
                        + (b0 + (self.edge_size + self.gap_size) * (a0 - b0))
                            * self.cell_scale[cell],
                ),
                self.calc_pos(
                    self.cell_pos[cell]
                        + (b0
                            + (self.edge_size + self.gap_size) * (a0 - b0)
                            + self.edge_size * (b1 - b0))
                            * self.cell_scale[cell],
                ),
                self.calc_pos(
                    self.cell_pos[cell]
                        + (a0
                            + (self.edge_size + self.gap_size) * (b0 - a0)
                            + self.edge_size * (a1 - a0))
                            * self.cell_scale[cell],
                ),
            ],
            color,
            PathStroke::NONE,
        ));
    }

    pub fn draw_vertex(&self, a: Vec2, b: Vec2, c: Vec2, cell: usize, color: Color32, ui: &mut Ui) {
        ui.painter().add(PathShape::convex_polygon(
            vec![
                self.calc_pos(self.cell_pos[cell] + b * self.cell_scale[cell]),
                self.calc_pos(
                    self.cell_pos[cell] + (b + (self.edge_size) * (c - b)) * self.cell_scale[cell],
                ),
                self.calc_pos(
                    self.cell_pos[cell]
                        + (b + (self.edge_size) * (c + a - 2.0 * b)) * self.cell_scale[cell],
                ),
                self.calc_pos(
                    self.cell_pos[cell] + (b + (self.edge_size) * (a - b)) * self.cell_scale[cell],
                ),
            ],
            color,
            PathStroke::NONE,
        ));
    }

    pub fn calc_pos(&self, v: Vec2) -> Pos2 {
        self.offset + (v * self.scale)
    }
}
