//! `ransi` has some console operations
//! 
//! * You can `use ransi::{AnsiText, AnsiColor}` if you wanna build ansi texts




use std::{process::Command, io::{stdout, Write}};



mod ansi_text;



pub use self::ansi_text::AnsiText;
pub use self::ansi_text::AnsiColor;



/// Clear the Console
/// 
/// Equivalent to `cls` in windows cmd
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



/// Move the cursor to the start of next line
pub fn cursor_next_line(n: i32) {
    print!("\x1B[{n}E");
    stdout().flush().unwrap();
}

/// Move the cursor to the start of previous line
pub fn cursor_previous_line(n: i32) {
    print!("\x1B[{n}F");
    stdout().flush().unwrap();
}



/// Erase the texts in display
pub fn erase_in_display(n: i32) {
    print!("\x1B[{n}J");
    stdout().flush().unwrap();
}

/// Erase the texts in line
pub fn erase_in_line(n: i32) {
    print!("\x1B[{n}K");
    stdout().flush().unwrap();
}



/// Set the cursor position
pub fn set_cursor_position(x: i32, y: i32) {
    print!("\x1B[{0};{1}H", y + 1, x + 1);
    stdout().flush().unwrap();
}

/// Save the cursor position in order to `restore` it later
pub fn save_cursor_position() {
    print!("\x1B[s");
    stdout().flush().unwrap();
}

/// Restore the cursor position to the `saved` position
pub fn restore_cursor_position() {
    print!("\x1B[u");
    stdout().flush().unwrap();
}



/// Show the cursor
pub fn show_cursor() {
    print!("\x1B[?25h");
    stdout().flush().unwrap();
}

/// Hide the cursor
pub fn hide_cursor() {
    print!("\x1B[?25l");
    stdout().flush().unwrap();
}
