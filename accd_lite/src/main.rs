use accd_gui;
use std::process;

fn main() {
    if let Err(e) = accd_gui::run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
