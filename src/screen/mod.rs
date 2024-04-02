use ggez::Context;
use ggez::context::Has;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Drawable, DrawParam, Mesh, PxScale, Quad, Rect, Text, TextFragment, TextLayout};
use ggez::mint::Point2;

use crate::constants::FONT_FLAG_NAME;
use crate::screen::char_line::{CharCode, TerminalCharColor};
use crate::screen::char_resolver::CharResolver;

mod char_resolver;
mod char_line;

const CHAR_CELL_SIZE: f32 = 28.;

pub struct Screen {
    width: u32,
    height: u32,
    cols: u32,
    rows: u32,
    vt_parser: vte::Parser,
    char_resolver: CharResolver,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        let (cols, rows) = Screen::revise_cell(width, height);
        Self {
            width,
            height,
            cols,
            rows,
            vt_parser: vte::Parser::new(),
            char_resolver: CharResolver::new(),
        }
    }

    pub fn load_buf(&mut self, buf: Box<[u8]>) {
        for byte in buf.iter() {
            self.vt_parser.advance(&mut self.char_resolver, *byte);
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        let (cols, rows) = Screen::revise_cell(width, height);
        self.cols = cols;
        self.rows = rows;
    }

    /// 根据宽高，计算当前屏幕所能呈现的字符格子数
    fn revise_cell(width: u32, height: u32) -> (u32, u32) {
        let cols = (width as f32 / CHAR_CELL_SIZE).floor() as u32;
        let rows = (height as f32 / CHAR_CELL_SIZE).floor() as u32;
        (cols, rows)
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &Context) {
        let lines = self.char_resolver.get_lines();
        let line_len = lines.len();
        if line_len == 0 {
            return;
        }
        let start_line_idx = if line_len > self.rows as usize {
            line_len - self.rows as usize
        } else {
            0
        };
        let end_line_idx = line_len - 1;
        for line_idx in start_line_idx..=end_line_idx {
            let renderable_line = &lines[line_idx];
            let char_codes = renderable_line.char_codes();
            let char_code_len = char_codes.len();
            if char_code_len == 0 {
                continue;
            }
            let start_code_idx = 0;
            let end_code_idx = if char_code_len > self.cols as usize {
                self.cols as usize - 1
            } else {
                char_code_len - 1
            };
            for col_idx in start_code_idx..=end_code_idx {
                // line_idx是一个大于等于0的，表示文本行列表的索引，
                // 这里将其减去 start_line_idx，才能得到屏幕上的垂直cell的索引
                let row_idx = line_idx - start_line_idx;
                let cc = &char_codes[col_idx];
                let rect = Rect::new(
                    col_idx as f32 * CHAR_CELL_SIZE,
                    row_idx as f32 * CHAR_CELL_SIZE,
                    CHAR_CELL_SIZE,
                    CHAR_CELL_SIZE,
                );
                self.draw_single_char_code(canvas, ctx, cc, rect);
            }
        }
    }

    fn draw_single_char_code(&self, canvas: &mut Canvas, ctx: &Context, cc: &CharCode, rect: Rect) {
        let fg_color = convert_color(&cc.style.fg_color);
        let mut txt = Text::new(TextFragment {
            text: cc.c.clone().to_string(),
            font: Some(FONT_FLAG_NAME.into()),
            scale: Some(PxScale { x: rect.w, y: rect.h }),
            color: Some(fg_color),
        });
        txt.set_bounds(Vec2::new(rect.w, rect.h)).set_layout(TextLayout::center());
        // 背景色
        if let Some(bg_color) = &cc.style.bg_color {
            let bg_color = convert_color(bg_color);
            canvas.draw(
                &Quad,
                DrawParam::default().dest(rect.point()).scale(rect.size()).color(bg_color),
            );
        }
        // 下划线
        if cc.style.underline {
            let start = Point2::from([rect.x, rect.y + rect.h]);
            let end = Point2::from([rect.x + rect.w, rect.y + rect.h]);
            let mesh = Mesh::new_line(ctx, &[start, end], 1., fg_color).expect("");
            canvas.draw(&mesh, DrawParam::default());
        }
        canvas.draw(
            &txt,
            DrawParam::default().dest(rect.center()),
        );
    }
}

fn convert_color(terminal_color: &TerminalCharColor) -> Color {
    let [r, g, b, a] = terminal_color.get_rgba();
    Color::from_rgba(r, g, b, a)
}