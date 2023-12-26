use::slint;

slint::slint!{ import { MainWindow } from "assets/ui/spec.slint";}
fn main() {
    let main_window = MainWindow::new().unwrap();
    main_window.run().unwrap();
}
