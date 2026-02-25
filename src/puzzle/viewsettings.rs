use eframe::egui::{Pos2, Vec2};

use crate::puzzle::setup11c;

pub struct ViewSettings {
    pub cell_pos: Vec<Vec2>,
    pub cell_scale: Vec<f32>,
    pub cell_outline: Vec<Vec2>,
    pub edge_size: f32,
    pub gap_size: f32,
    pub alt_ridge_width: f32,
    pub scale: f32,
    pub offset: Pos2,
}

impl ViewSettings {
    pub fn default() -> Self {
        Self {
            cell_pos: setup11c::cell_positions(),
            cell_scale: setup11c::cell_scales(),
            cell_outline: setup11c::cell_outline(),
            edge_size: 0.13,
            gap_size: 0.07,
            alt_ridge_width: 0.05,
            scale: 120.0,
            offset: Pos2::new(600.0, 540.0),
        }
    }
}
