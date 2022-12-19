use crate::frontend::main_page;
mod frontend;
use iced::{Settings, Application};

fn main() {
    println!("Hello, world!");
    main_page::MainWindow::run(Settings::default());
}

