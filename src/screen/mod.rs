use ggez::context::Has;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Drawable, DrawParam, GraphicsContext, PxScale, Rect, Text, TextFragment, TextLayout, Transform};
use crate::constants::FONT_FLAG_NAME;
use crate::screen::char_line::CharCode;
use crate::screen::char_resolver::CharResolver;

mod char_resolver;
mod char_line;

const CHAR_SIZE: f32 = 28.;

pub struct Screen {
    width: u32,
    height: u32,
    vt_parser: vte::Parser,
    char_resolver: CharResolver,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            vt_parser: vte::Parser::new(),
            char_resolver: CharResolver::new(),
        }
    }

    pub fn load_buf(&mut self, buf: Box<[u8]>) {
        for byte in buf.iter() {
            self.vt_parser.advance(&mut self.char_resolver, *byte);
        }
    }
}

impl Drawable for Screen {
    fn draw(&self, canvas: &mut Canvas, _param: impl Into<DrawParam>) {
        let lines = self.char_resolver.get_lines();
        let line_len = lines.len();
        for row_idx in 0..line_len {
            let line = &lines[row_idx];
            let char_codes = line.char_codes();
            let char_len = char_codes.len();
            for col_idx in 0..char_len {
                let cc = &char_codes[col_idx];
                let rect = Rect::new(
                    col_idx as f32 * CHAR_SIZE,
                    row_idx as f32 * CHAR_SIZE,
                    CHAR_SIZE,
                    CHAR_SIZE,
                );
                draw_text(canvas, cc, rect);
            }
        }

        fn draw_text(cv: &mut Canvas, cc: &CharCode, rect: Rect) {
            let mut txt = Text::new(TextFragment {
                text: cc.c.clone().to_string(),
                font: Some(FONT_FLAG_NAME.into()),
                scale: Some(PxScale { x: rect.w, y: rect.h }),
                color: Some(Color::WHITE),
            });
            txt.set_bounds(Vec2::new(rect.w, rect.h)).set_layout(TextLayout::center());
            cv.draw(
                &txt,
                DrawParam::default().dest(rect.center()),
            );
        }
    }

    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        None
    }
}

