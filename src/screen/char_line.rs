pub struct CharLine {
    char_codes: Vec<CharCode>,
}

impl CharLine {
    pub fn new() -> Self {
        Self {
            char_codes: Vec::new()
        }
    }

    pub fn push_code(&mut self, char_code: CharCode) {
        self.char_codes.push(char_code)
    }

    pub fn char_codes(&self) -> &Vec<CharCode> {
        &self.char_codes
    }
}

pub struct CharCode {
    pub c: char,
    pub style: CharCodeStyle,
}

impl CharCode {
    pub fn new(c: char, style: CharCodeStyle) -> Self {
        Self {
            c,
            style,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TerminalCharColor([u8; 4]);

impl TerminalCharColor {
    pub const WHITE: TerminalCharColor = TerminalCharColor([255, 255, 255, 255]);
    pub const BLACK: TerminalCharColor = TerminalCharColor([0, 0, 0, 255]);
    pub const RED: TerminalCharColor = TerminalCharColor([255, 0, 0, 255]);
    pub const GREEN: TerminalCharColor = TerminalCharColor([0, 255, 0, 255]);
    pub const BLUE: TerminalCharColor = TerminalCharColor([0, 0, 255, 255]);
    pub const YELLOW: TerminalCharColor = TerminalCharColor([255, 255, 0, 255]);
    pub const PURPLE: TerminalCharColor = TerminalCharColor([128, 0, 128, 255]);
    pub const CYAN: TerminalCharColor = TerminalCharColor([0, 255, 255, 255]);

    pub fn get_rgba(&self) -> [u8; 4] {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct CharCodeStyle {
    pub bold: bool,
    pub underline: bool,
    pub fg_color: TerminalCharColor,
    pub bg_color: Option<TerminalCharColor>,
}

impl CharCodeStyle {
    pub fn new() -> Self {
        Self {
            bold: false,
            underline: false,
            fg_color: TerminalCharColor::WHITE,
            bg_color: None,
        }
    }
}