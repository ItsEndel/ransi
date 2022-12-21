
pub enum AnsiColor {
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE
}



impl AnsiColor {
    pub fn get_code(&self, is_bg: bool, is_bright: bool) -> i32 {
        let mut code: i32 = 30 + 
        match self {
            AnsiColor::BLACK => 0,
            AnsiColor::RED => 1,
            AnsiColor::GREEN => 2,
            AnsiColor::YELLOW => 3,
            AnsiColor::BLUE => 4,
            AnsiColor::MAGENTA => 5,
            AnsiColor::CYAN => 6,
            AnsiColor::WHITE => 7
        };

        if is_bg { code += 10 };
        if is_bright { code += 60 };

        return code;
    }
}
