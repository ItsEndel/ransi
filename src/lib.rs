use std::{process::Command, io::{stdout, Write}};



pub mod ansi_text;
pub mod ansi_color;



pub use ansi_text::AnsiText;
pub use ansi_color::AnsiColor;



pub fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "cls"])
                .status()
                .expect("Screen Cleaning Failed");
    } else {
        Command::new("sh")
                .args(["-c", "clear"])
                .status()
                .expect("Screen Cleaning Failed");
    }
}



pub fn cursor_next_line(n: i32) {
    print!("\x1B[{n}E");
    stdout().flush().unwrap();
}

pub fn cursor_previous_line(n: i32) {
    print!("\x1B[{n}F");
    stdout().flush().unwrap();
}



pub fn erase_in_display(n: i32) {
    print!("\x1B[{n}J");
    stdout().flush().unwrap();
}

pub fn erase_in_line(n: i32) {
    print!("\x1B[{n}K");
    stdout().flush().unwrap();
}



pub fn set_cursor_position(x: i32, y: i32) {
    print!("\x1B[{0};{1}H", y + 1, x + 1);
    stdout().flush().unwrap();
}

pub fn save_cursor_position() {
    print!("\x1B[s");
    stdout().flush().unwrap();
}

pub fn restore_cursor_position() {
    print!("\x1B[u");
    stdout().flush().unwrap();
}



pub fn show_cursor() {
    print!("\x1B[?25h");
    stdout().flush().unwrap();
}

pub fn hide_cursor() {
    print!("\x1B[?25l");
    stdout().flush().unwrap();
}
