use vte::{Params, Perform};
use crate::screen::char_line::{CharCode, CharLine, TerminalCharColor};

pub struct CharResolver {
    char_lines: Vec<CharLine>,
}

impl CharResolver {
    pub fn new() -> Self {
        Self {
            char_lines: Vec::new()
        }
    }

    pub fn get_lines(&self) -> &[CharLine] {
        &self.char_lines
    }
}

impl Perform for CharResolver {
    fn print(&mut self, c: char) {
        if let Some(last_line) = self.char_lines.last_mut() {
            let c = if c == '\u{0000}' { ' ' } else { c };
            last_line.push_code(CharCode::new(c));
        }
    }

    fn execute(&mut self, byte: u8) {
        if byte == 0x0A {
            // /r/n => 0x0D,0x0A，新一行
            let new_line = CharLine::new();
            self.char_lines.push(new_line);
        }
    }

    fn csi_dispatch(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, action: char) {
        match action {
            // 在ANSI转义序列中，以 m 结尾的一般是指文本样式、颜色或其他可视属性的设置。
            'm' => {}
            _ => {}
        }
    }
}

impl CharResolver {
    fn get_terminal_color(color_num: u32) -> TerminalCharColor {
        match color_num {
            // 30前景色黑色，40背景色黑色
            30 | 40 => TerminalCharColor::BLACK,
            31 | 41 => TerminalCharColor::RED,
            32 | 42 => TerminalCharColor::GREEN,
            33 | 43 => TerminalCharColor::YELLOW,
            34 | 44 => TerminalCharColor::BLUE,
            35 | 45 => TerminalCharColor::PURPLE,
            36 | 46 => TerminalCharColor::CYAN,
            37 | 47 | _ => TerminalCharColor::WHITE,
        }
    }
}