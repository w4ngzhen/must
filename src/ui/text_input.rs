use ggez::graphics::{
    Canvas, Color, DrawMode, DrawParam, FillOptions, Mesh, MeshBuilder, Rect, StrokeOptions,
};
use ggez::{Context, GameResult};

pub struct TextInput {
    focused: bool,
    value: String,
    history: Vec<String>,
    cursor: u16,
    bounds: Rect,
}

impl TextInput {
    pub fn new(default_value: String, bounds: Rect) -> Self {
        Self {
            focused: true,
            value: default_value,
            history: Vec::new(),
            cursor: 0,
            bounds,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult<()> {
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
        Ok(())
    }

    pub fn update_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
}
