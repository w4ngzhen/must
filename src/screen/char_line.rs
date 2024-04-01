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
}

impl CharCode {
    pub fn new(c: char) -> Self {
        Self {
            c
        }
    }
}