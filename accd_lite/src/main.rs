use std::process;
use accd_gui;

fn main() {
    if let Err(e) = accd_gui::run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
