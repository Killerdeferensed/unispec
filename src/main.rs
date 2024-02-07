use slint::{ComponentHandle, Model};
pub mod file_loader;
mod peak;
mod roi;
mod spec;
mod tests;
mod data_processing;

slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();
    main_window.run().unwrap();
}
