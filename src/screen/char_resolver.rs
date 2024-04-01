use vte::{Params, Perform};
use crate::screen::char_line::{CharCode, CharLine};

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

    fn csi_dispatch(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: char) {}
}