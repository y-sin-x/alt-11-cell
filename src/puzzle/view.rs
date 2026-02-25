use eframe::{
    egui::{Color32, Pos2, Ui, Vec2},
    epaint::{PathShape, PathStroke},
};

use crate::puzzle::{setup11c, state::PuzzleState, twist::Twist, viewsettings::ViewSettings};

pub struct Polygon {
    pub vertices: Vec<Vec2>,
}

impl Polygon {
    // only works if polygon is convex
    pub fn contains_pos(&self, pos: Vec2) -> bool {
        let n = self.vertices.len();
        let mut v1 = self.vertices[0] - self.vertices[n - 1];
        let mut v2 = pos - self.vertices[n - 1];
        let dir = v1.x * v2.y - v1.y * v2.x > 0.0;
        for i in 0..n - 1 {
            v1 = self.vertices[i + 1] - self.vertices[i];
            v2 = pos - self.vertices[i];
            if (v1.x * v2.y - v1.y * v2.x > 0.0) != dir {
                return false;
            }
        }
        true
    }
}

pub struct Face {
    pub polygon: Polygon,
    pub grip: usize,
    pub mirrored: bool,
}

impl Face {
    pub fn new(vertices: Vec<Vec2>, grip: usize, mirrored: bool) -> Self {
        Self {
            polygon: Polygon { vertices },
            grip,
            mirrored,
        }
    }
}

pub enum Substicker<'a> {
    Ridge(&'a Vec<Vec2>),
    Edge(Vec2, Vec2, Vec2, Vec2),
    Vertex(Vec2, Vec2, Vec2),
    None,
}

pub struct PuzzleView {
    pub state: PuzzleState,
    pub was_scrambled: bool,
    pub alt_view: bool,
    pub filters: Vec<Vec<u8>>,
    pub filter_idx: usize,
    pub faces: Vec<Face>,
    pub settings: ViewSettings,
}

impl PuzzleView {
    pub fn show_puzzle(&mut self, ui: &mut Ui) {
        self.draw_cells(ui);
        for piece in &self.state.pieces {
            for cell in 0..piece.degree() {
                let centered_piece = piece.rotate(&setup11c::cell_recenter(cell));
                for face in &self.faces {
                    if centered_piece.grip_state(face.grip) == 1 {
                        let face_piece = centered_piece.rotate(&setup11c::face_recenter(face.grip));
                        let secondary = setup11c::cell_recenter(cell).inverse().permute(face.grip);
                        let v = &face.polygon.vertices;
                        let inv_att = piece.att.inverse().clone();
                        let mut color = if self.alt_view {
                            setup11c::colors(inv_att.permute(secondary))
                        } else {
                            setup11c::colors(inv_att.permute(cell))
                        };
                        for g in 0..piece.degree() {
                            if piece.grip_state(g) == 1
                                && self.filters[self.filter_idx][inv_att.permute(g)] == 0
                            {
                                color = Color32::from_rgb(40, 40, 40);
                                break;
                            }
                        }

                        match setup11c::substicker(face_piece, v) {
                            Substicker::Ridge(v) => {
                                self.draw_ridge(
                                    v,
                                    cell,
                                    self.settings.edge_size + self.settings.gap_size,
                                    color,
                                    ui,
                                );
                                if self.alt_view {
                                    self.draw_ridge(
                                        v,
                                        cell,
                                        self.settings.edge_size
                                            + self.settings.gap_size
                                            + self.settings.alt_ridge_width,
                                        setup11c::colors(secondary),
                                        ui,
                                    );
                                }
                            }
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
        let s = &self.settings;
        for cell in 0..self.state.degree {
            if pos.distance(self.calc_pos(cell, Vec2::ZERO)) < s.scale * s.cell_scale[cell].abs() {
                let rel_pos = ((pos - s.offset) / s.scale - s.cell_pos[cell]) / s.cell_scale[cell];
                let recenter = setup11c::cell_recenter(cell);

                for face in &self.faces {
                    if face.polygon.contains_pos(rel_pos) {
                        let face_recenter = setup11c::face_recenter(face.grip);

                        let mut rot = setup11c::face_rot(ccw ^ face.mirrored);
                        let v = &face.polygon.vertices;
                        let n = v.len();
                        if self.edge_poly(v, 0).contains_pos(rel_pos)
                            && self.edge_poly(v, n - 1).contains_pos(rel_pos)
                        {
                            rot = setup11c::vertex_rot(ccw ^ face.mirrored);
                        } else {
                            for i in 0..n {
                                if self.edge_poly(v, i).contains_pos(rel_pos) {
                                    let align = setup11c::face_rot(false).exp(i + 1);
                                    if self.edge_poly(v, i + 1).contains_pos(rel_pos) {
                                        rot = align
                                            .product(&setup11c::vertex_rot(ccw ^ face.mirrored))
                                            .product(&align.inverse());
                                    } else {
                                        rot = align
                                            .product(&setup11c::edge_rot())
                                            .product(&align.inverse());
                                    }
                                    break;
                                }
                            }
                        }
                        self.state.twist_move(&Twist {
                            grip: cell,
                            rot: recenter
                                .product(&face_recenter)
                                .product(&rot)
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

    pub fn draw_ridge(&self, v: &Vec<Vec2>, cell: usize, margin: f32, color: Color32, ui: &mut Ui) {
        let n = v.len();

        let mut points = Vec::new();
        for i in 0..n {
            points.push(self.calc_pos(
                cell,
                v[i] + margin * (v[(i + 1) % n] + v[(i + 2) % n] - 2.0 * v[i]),
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
        let edge = self.settings.edge_size;
        let gap = self.settings.gap_size;
        ui.painter().add(PathShape::convex_polygon(
            vec![
                self.calc_pos(cell, a0 + (edge + gap) * (b0 - a0)),
                self.calc_pos(cell, b0 + (edge + gap) * (a0 - b0)),
                self.calc_pos(cell, b0 + (edge + gap) * (a0 - b0) + edge * (b1 - b0)),
                self.calc_pos(cell, a0 + (edge + gap) * (b0 - a0) + edge * (a1 - a0)),
            ],
            color,
            PathStroke::NONE,
        ));
    }

    pub fn draw_vertex(&self, a: Vec2, b: Vec2, c: Vec2, cell: usize, color: Color32, ui: &mut Ui) {
        ui.painter().add(PathShape::convex_polygon(
            vec![
                self.calc_pos(cell, b),
                self.calc_pos(cell, b + self.settings.edge_size * (c - b)),
                self.calc_pos(cell, b + self.settings.edge_size * (c + a - 2.0 * b)),
                self.calc_pos(cell, b + self.settings.edge_size * (a - b)),
            ],
            color,
            PathStroke::NONE,
        ));
    }

    pub fn draw_cells(&self, ui: &mut Ui) {
        for cell in 0..self.state.degree {
            ui.painter().circle_filled(
                self.calc_pos(cell, Vec2::ZERO),
                self.settings.cell_scale[cell].abs() * self.settings.scale,
                setup11c::colors(cell),
            );
            ui.painter().add(PathShape::convex_polygon(
                self.settings
                    .cell_outline
                    .iter()
                    .map(|p| self.calc_pos(cell, *p))
                    .collect(),
                Color32::BLACK,
                PathStroke::NONE,
            ));
        }
    }

    pub fn edge_poly(&self, v: &Vec<Vec2>, i: usize) -> Polygon {
        let n = v.len();
        Polygon {
            vertices: vec![
                v[i % n],
                v[(i + 1) % n],
                v[(i + 1) % n]
                    + (self.settings.edge_size + self.settings.gap_size)
                        * (v[(i + 2) % n] - v[(i + 1) % n]),
                v[i % n]
                    + (self.settings.edge_size + self.settings.gap_size)
                        * (v[(i + n - 1) % n] - v[i % n]),
            ],
        }
    }

    pub fn calc_pos(&self, cell: usize, v: Vec2) -> Pos2 {
        let s = &self.settings;
        s.offset + ((s.cell_pos[cell] + v * s.cell_scale[cell]) * s.scale)
    }
}
