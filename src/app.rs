use eframe::egui::{self, Event, PointerButton, Pos2};

use crate::puzzle::{setup11c, state::PuzzleState, view::PuzzleView};

pub struct App {
    puzzle: PuzzleView,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        App {
            puzzle: PuzzleView {
                state: PuzzleState::generate(&setup11c::base_pieces(), &setup11c::generators()),
                was_scrambled: false,
                cell_pos: setup11c::cell_positions(),
                cell_scale: setup11c::cell_scales(),
                faces: setup11c::faces(),
                alt_view: false,
                edge_size: 0.1,
                gap_size: 0.1,
                scale: 100.0,
                offset: Pos2::new(600.0, 500.0),
            },
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.puzzle.was_scrambled && self.puzzle.state.is_solved {
                ui.label("Solved!");
            }

            let events = ui.input(|i| i.events.clone());

            for e in events {
                if let Event::PointerButton {
                    pos,
                    button,
                    pressed: true,
                    modifiers: _,
                } = e
                {
                    if button == PointerButton::Primary {
                        self.puzzle.pointer_twist(pos, true);
                    } else if button == PointerButton::Secondary {
                        self.puzzle.pointer_twist(pos, false);
                    }
                }
            }

            if ui.input(|i| i.key_pressed(egui::Key::Space)) {
                self.puzzle.alt_view ^= true;
            }

            if ui.input(|i| i.key_pressed(egui::Key::S)) {
                self.puzzle.pointer_twist(
                    ui.input(|i| i.pointer.latest_pos().unwrap_or(Pos2::default())),
                    true,
                );
            }

            if ui.input(|i| i.key_pressed(egui::Key::D)) {
                self.puzzle.pointer_twist(
                    ui.input(|i| i.pointer.latest_pos().unwrap_or(Pos2::default())),
                    false,
                );
            }

            if ui.input(|i| i.modifiers.ctrl) {
                if ui.input(|i| i.key_pressed(egui::Key::R)) {
                    self.puzzle.state.reset();
                    self.puzzle.was_scrambled = false;
                }

                if ui.input(|i| i.key_pressed(egui::Key::F)) {
                    self.puzzle.state.reset();
                    self.puzzle.state.scramble(1000);
                    self.puzzle.was_scrambled = true;
                }

                if ui.input(|i| i.key_pressed(egui::Key::Z)) {
                    self.puzzle.state.undo();
                }
            }

            self.puzzle.show_puzzle(ui);
            ctx.request_repaint();
        });
    }
}
