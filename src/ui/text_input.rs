use ggez::graphics::{
    Canvas, Color, DrawMode, DrawParam, FillOptions, Mesh, MeshBuilder, PxScale, Rect,
    StrokeOptions, Text, TextFragment, TextLayout,
};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::constants::FONT_FLAG_NAME;
use crate::utils::is_wide_char;

pub struct TextInput {
    focused: bool,
    value: String,
    history: Vec<String>,
    cursor: u16,
    bounds: Rect,
    padding: f32,
}

impl TextInput {
    pub fn new(default_value: String, bounds: Rect) -> Self {
        Self {
            focused: true,
            value: default_value,
            history: Vec::new(),
            cursor: 0,
            bounds,
            padding: 5.,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult<()> {
        // 1. 控件边框
        let mut builder = MeshBuilder::new();
        let mesh_data = builder
            .rectangle(
                DrawMode::Fill(FillOptions::default()),
                self.bounds,
                Color::BLACK,
            )?
            .rounded_rectangle(
                DrawMode::Stroke(StrokeOptions::default()),
                self.bounds,
                5.0,
                Color::WHITE,
            )?
            .build();
        let mesh = Mesh::from_data(ctx, mesh_data);
        canvas.draw(&mesh, DrawParam::default());
        // 2. 绘制文本
        self.draw_text(canvas);
        Ok(())
    }

    fn draw_text(&self, canvas: &mut Canvas) {
        let char_cell_wide_width = 28.;
        let char_cell_thin_width = 24.;
        let render_bounds = Rect::from([
            self.bounds.x + self.padding,
            self.bounds.y + self.padding,
            self.bounds.w - self.padding * 2.,
            self.bounds.h - self.padding * 2.,
        ]);
        // 计算文本宽度，这里使用格子来填充每一个字符
        let mut text_width = 0f32;
        let mut text_overflow: bool = false;
        for c in self.value.chars() {
            let char_width = if is_wide_char(c) {
                char_cell_wide_width
            } else {
                char_cell_thin_width
            };
            text_width = text_width + char_width;
            if text_width > render_bounds.w {
                text_overflow = true;
                break;
            }
        }
        if text_overflow {
            let mut next_char_x = render_bounds.x + render_bounds.w;
            // 反向取字符，从尾巴开始绘制
            for c in self.value.chars().rev() {
                let char_width = if is_wide_char(c) {
                    char_cell_wide_width
                } else {
                    char_cell_thin_width
                };
                next_char_x -= char_width;
                if next_char_x <= 0. {
                    break;
                }
                let text_rect =
                    Rect::new(next_char_x, render_bounds.y, char_width, render_bounds.h);
                let mut txt = Text::new(TextFragment {
                    text: c.to_string(),
                    font: Some(FONT_FLAG_NAME.into()),
                    ..Default::default()
                });
                txt.set_bounds(text_rect.size())
                    .set_scale(PxScale {
                        x: char_width,
                        y: render_bounds.h,
                    })
                    .set_layout(TextLayout::center());
                canvas.draw(
                    &txt,
                    DrawParam::default().dest(Point2::from(text_rect.center())),
                );
            }
        } else {
            let mut next_char_x = render_bounds.x;
            // 正向
            for c in self.value.chars() {
                let char_width = if is_wide_char(c) {
                    char_cell_wide_width
                } else {
                    char_cell_thin_width
                };
                let text_rect =
                    Rect::new(next_char_x, render_bounds.y, char_width, render_bounds.h);
                let mut txt = Text::new(TextFragment {
                    text: c.to_string(),
                    font: Some(FONT_FLAG_NAME.into()),
                    ..Default::default()
                });
                txt.set_bounds(text_rect.size())
                    .set_scale(PxScale {
                        x: char_width,
                        y: render_bounds.h,
                    })
                    .set_layout(TextLayout::center());
                canvas.draw(
                    &txt,
                    DrawParam::default().dest(Point2::from(text_rect.center())),
                );
                next_char_x += char_width;
            }
        }
    }

    pub fn update_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn append_char(&mut self, c: char) {
        self.value.push(c);
    }

    pub fn delete_last_char(&mut self) {
        self.value.pop();
    }
}
