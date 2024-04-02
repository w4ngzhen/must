use vte::{Params, Perform};
use crate::screen::char_line::{CharCode, CharCodeStyle, CharLine, TerminalCharColor};

pub struct CharResolver {
    char_lines: Vec<CharLine>,
    current_code_style: CharCodeStyle,
}

impl CharResolver {
    pub fn new() -> Self {
        Self {
            char_lines: Vec::new(),
            current_code_style: CharCodeStyle::new(),
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
            last_line.push_code(CharCode::new(c, self.current_code_style.clone()));
        }
    }

    fn execute(&mut self, byte: u8) {
        if byte == 0x0A {
            // /r/n => 0x0D,0x0A，新一行
            let new_line = CharLine::new();
            self.char_lines.push(new_line);
        }
    }

    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, action: char) {
        match action {
            // 在ANSI转义序列中，以 m 结尾的一般是指文本样式、颜色或其他可视属性的设置。
            'm' => {
                self.current_code_style = self.resolve_style(params);
            }
            _ => {}
        }
    }
}

impl CharResolver {
    const fn get_terminal_color(color_num: u16) -> TerminalCharColor {
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

    fn resolve_style(&self, params: &Params) -> CharCodeStyle {
        let mut style = CharCodeStyle::new();
        for param in params.iter() {
            if param.len() == 0 {
                continue;
            }
            for sub_param in param.iter() {
                let val = *sub_param;
                if val >= 30 && val <= 37 {
                    style.fg_color = CharResolver::get_terminal_color(val)
                } else if val >= 40 && val <= 47 {
                    style.bg_color = Some(CharResolver::get_terminal_color(val))
                } else if val == 4 {
                    style.underline = true;
                } else if val == 24 {
                    style.underline = false;
                } else if val == 1 {
                    style.bold = true;
                } else if val == 22 {
                    style.bold = false;
                } else if val == 0 {
                    style = CharCodeStyle::new()
                }
            }
        }
        style
    }
}