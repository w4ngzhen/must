use ggez::context::Has;
use ggez::glam::Vec2;
use ggez::graphics::{
    Canvas, Color, DrawParam, Drawable, Mesh, PxScale, Quad, Rect, Text, TextFragment, TextLayout,
};
use ggez::mint::Point2;
use ggez::Context;

use crate::constants::FONT_FLAG_NAME;
use crate::screen::char_line::{CharCode, TerminalCharColor};
use crate::screen::char_resolver::CharResolver;

mod char_line;
mod char_resolver;

const CHAR_CELL_HEIGHT: f32 = 28.;
const CHAR_CELL_WIDE_WIDTH: f32 = 28.;
const CHAR_CELL_THIN_WIDTH: f32 = 24.;

pub struct Screen {
    bounds: Rect,
    rows: u32,
    vt_parser: vte::Parser,
    char_resolver: CharResolver,
}

impl Screen {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            rows: (bounds.h / CHAR_CELL_HEIGHT).floor() as u32,
            vt_parser: vte::Parser::new(),
            char_resolver: CharResolver::new(),
        }
    }

    pub fn load_buf(&mut self, buf: Box<[u8]>) {
        for byte in buf.iter() {
            self.vt_parser.advance(&mut self.char_resolver, *byte);
        }
    }

    pub fn update_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.rows = (bounds.h / CHAR_CELL_HEIGHT).floor() as u32;
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
            // line_idx是一个大于等于0的，表示文本行列表的索引，
            // 这里将其减去 start_line_idx，才能得到屏幕上的垂直cell的索引
            let row_idx = line_idx - start_line_idx;
            // 已经渲染的字符的总宽度，
            let mut rendered_char_width = 0f32;
            for char_idx in 0..char_code_len {
                let cc = &char_codes[char_idx];
                // 如果cc为宽度字符，用更宽的格子呈现
                let char_width = if is_wide_char(cc.c) {
                    CHAR_CELL_WIDE_WIDTH
                } else {
                    CHAR_CELL_THIN_WIDTH
                };
                if rendered_char_width + char_width > self.bounds.w {
                    // 若该字符待渲染的宽度加上前面已经渲染的宽度超过了当前终端画布宽度，不再渲染后续内容
                    break;
                }
                let rect = Rect::new(
                    self.bounds.x + rendered_char_width,
                    self.bounds.y + (row_idx as f32 * CHAR_CELL_HEIGHT),
                    char_width,
                    CHAR_CELL_HEIGHT,
                );
                self.draw_single_char_code(canvas, ctx, cc, rect);
                rendered_char_width += char_width;
            }
        }
    }

    fn draw_single_char_code(&self, canvas: &mut Canvas, ctx: &Context, cc: &CharCode, rect: Rect) {
        let fg_color = convert_color(&cc.style.fg_color);
        let mut txt = Text::new(TextFragment {
            text: cc.c.clone().to_string(),
            font: Some(FONT_FLAG_NAME.into()),
            scale: Some(PxScale {
                x: rect.w,
                y: rect.h,
            }),
            color: Some(fg_color),
        });
        txt.set_bounds(Vec2::new(rect.w, rect.h))
            .set_layout(TextLayout::center());
        // 背景色
        if let Some(bg_color) = &cc.style.bg_color {
            let bg_color = convert_color(bg_color);
            canvas.draw(
                &Quad,
                DrawParam::default()
                    .dest(rect.point())
                    .scale(rect.size())
                    .color(bg_color),
            );
        }
        // 下划线
        if cc.style.underline {
            let start = Point2::from([rect.x, rect.y + rect.h]);
            let end = Point2::from([rect.x + rect.w, rect.y + rect.h]);
            let mesh = Mesh::new_line(ctx, &[start, end], 1., fg_color).expect("");
            canvas.draw(&mesh, DrawParam::default());
        }
        canvas.draw(&txt, DrawParam::default().dest(rect.center()));
    }
}

fn convert_color(terminal_color: &TerminalCharColor) -> Color {
    let [r, g, b, a] = terminal_color.get_rgba();
    Color::from_rgba(r, g, b, a)
}

/// 判断一个unicode字符是否是宽体的字符
/// 我们认为一个中文字符算宽体字符
fn is_wide_char(c: char) -> bool {
    (c >= '\u{4E00}' && c <= '\u{9FFF}')  // 基本多文种平面的CJK统一汉字块
        || (c >= '\u{3400}' && c <= '\u{4DBF}') // 其他补充区汉字块
}
