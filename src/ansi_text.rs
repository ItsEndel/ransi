//! `ansi_text` has a struct to let you build ansi texts quickly
//! 
//! * You can `use ransi::AnsiText` to import the struct
//! 
//! # Examples
//! 
//! ```
//! let mut ansi = AnsiText::new();
//! 
//! let text1 = ansi
//!     .bold()
//!     .italic()
//!     .underline()
//!     .color(AnsiColor::RED, false, false)
//!     .text("ERROR")
//!     .reset()
//!     .build();
//! 
//! let text2 = ansi
//!     .color(AnsiColor::GREEN, false, false)
//!     .text("GREEN ")
//!     .color(AnsiColor::YELLOW, false, false)
//!     .text("YELLOW ")
//!     .color_rgb(255, 0, 0, false)
//!     .text("GREAT RED")
//!     .reset()
//!     .build();
//! ```



mod ansi_color;



pub use ansi_color::AnsiColor;



/// Ansi Text Builder
/// 
/// # Examples
/// 
/// ```
/// let mut ansi = AnsiText::new();
/// 
/// let text = ansi
///     .color(AnsiColor::BLACK, true, false)
///     .color(AnsiColor::GREEN, false, false)
///     .text("Im Hacking Your Computer!")
///     .build();
/// ```
pub struct AnsiText {
    text: String,

    pushed_type: char,
    pushed_value: String,
}



impl AnsiText {
    /// Create a new `AnsiTextBuilder` instance
    pub fn new() -> AnsiText {
        AnsiText {
            text: String::from(""),
            pushed_type: ' ',
            pushed_value: String::from(""),
        }
    }
}



impl AnsiText {
    fn flush(&mut self) {
        // 如果上一次推入的类型不是纯文本
        if self.pushed_type != ' ' {
            // 如果已推入参数值的最后一个字符是分号
            if self.pushed_value.ends_with(';') {
                // 替换最后的分号为转义类型
                let range = self.pushed_value.chars().count() - 1;
                self.pushed_value.replace_range(range.., &self.pushed_type.to_string());

                // 在已推入参数值前面添加转义符
                self.pushed_value = String::from("\x1B[") + &self.pushed_value;
            }
        }

        // 推入文字
        self.text += &self.pushed_value;
    }

    fn push(&mut self, mode: char, value: &str) {
        if mode != self.pushed_type {
            self.switch_push_type(mode);
        }

        self.push_value(value);
    }

    fn push_value(&mut self, value: &str) {
        self.pushed_value += value;
    }

    fn switch_push_type(&mut self, mode: char) {
        // 推入文字
        self.flush();

        // 重置推入类型和推入参数值
        self.pushed_value = String::from("");
        self.pushed_type = mode;
    }



    fn clear(&mut self) {
        self.text = String::from("");

        self.pushed_type = ' ';
        self.pushed_value = String::from("");
    }
}



impl AnsiText {
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.push(' ', text);

        return self;
    }



    pub fn reset(&mut self) -> &mut Self {
        self.push('m', "0;");

        return self;
    }



    pub fn bold(&mut self) -> &mut Self {
        self.push('m', "1;");

        return self;
    }

    pub fn italic(&mut self) -> &mut Self {
        self.push('m', "3;");

        return self;
    }

    pub fn underline(&mut self) -> &mut Self {
        self.push('m', "4;");

        return self;
    }



    pub fn color(&mut self, color: AnsiColor, is_bg: bool, is_bright: bool) -> &mut Self {
        self.push('m', &format!("{0};", color.get_code(is_bg, is_bright)));

        return self;
    }

    pub fn color_palette(&mut self, color: u8, is_bg: bool) -> &mut Self {
        let mut code: i32 = 38;

        if is_bg { code += 10 };

        self.push('m', &format!("{code};5;{color};"));

        return self;
    }

    pub fn color_rgb(&mut self, r: u8, g: u8, b: u8, is_bg: bool) -> &mut Self {
        let mut code: i32 = 38;

        if is_bg { code += 10 };

        self.push('m', &format!("{code};2;{r};{g};{b};"));

        return self;
    }
}



impl AnsiText {
    pub fn build(&mut self) -> String {
        self.flush();

        let result = self.text.clone();

        self.clear();

        return result;
    }
}
