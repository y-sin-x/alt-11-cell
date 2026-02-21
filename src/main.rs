// thanks to henrydukepickle and HactarCE for much of this code + the concepts behind it

mod app;
mod puzzle;

use crate::app::App;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Alternative 11-Cell",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
