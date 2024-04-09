use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{Color, Rect};
use ggez::input::keyboard::{KeyInput, KeyMods};
use ggez::mint::Point2;
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context, GameError, GameResult};
use telnet::Telnet;

use crate::screen::Screen;
use crate::ui::text_input::TextInput;

pub struct GameState {
    telnet_client: Telnet,
    screen: Screen,
    text_input: TextInput,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> Self {
        let size = _ctx.gfx.window().inner_size();
        let (screen_bounds, input_bounds) =
            get_screen_and_input_bounds(size.width as f32, size.height as f32);
        let mut telnet_client =
            Telnet::connect(("pkuxkx.net", 8081), 1024).expect("Couldn't connect to the server...");
        Self {
            telnet_client,
            screen: Screen::new(screen_bounds),
            text_input: TextInput::new("hello, world.你好，世界。".into(), input_bounds),
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let event = self.telnet_client.read_nonblocking().expect("Read error");
        if let telnet::Event::Data(buffer) = event {
            self.screen.load_buf(buffer);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.screen.draw(&mut canvas, ctx);
        self.text_input.draw(&mut canvas, ctx)?;
        // Draw code here...
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.text_input
            .set_focused(self.text_input.bounds().contains(Point2::from([_x, _y])));
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        if self.text_input.focused() {
            match input {
                KeyInput {
                    keycode: Some(code),
                    mods,
                    ..
                } => match code {
                    VirtualKeyCode::Escape => self.text_input.set_focused(false),
                    VirtualKeyCode::Delete | VirtualKeyCode::Back => {
                        if KeyMods::CTRL == mods {
                            self.text_input.delete_char(5)
                        } else {
                            self.text_input.delete_char(1)
                        }
                    }
                    VirtualKeyCode::Return | VirtualKeyCode::Caret => {
                        if let Some(txt_str) = self.text_input.commit() {
                            self.telnet_client
                                .write(txt_str.as_bytes())
                                .expect("write data err.");
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> Result<(), GameError> {
        if self.text_input.focused() {
            // 控制键在key_down_event中处理，这里仅处理提交内容
            if _character.is_ascii_graphic() {
                // 目前仅支持ascii中的可见字符
                self.text_input.append_char(_character);
            }
        }
        Ok(())
    }

    fn resize_event(
        &mut self,
        _ctx: &mut Context,
        width: f32,
        height: f32,
    ) -> Result<(), GameError> {
        let (screen_bounds, input_bounds) = get_screen_and_input_bounds(width, height);
        self.screen.update_bounds(screen_bounds);
        self.text_input.update_bounds(input_bounds);
        Ok(())
    }
}

fn get_screen_and_input_bounds(window_width: f32, window_height: f32) -> (Rect, Rect) {
    let input_height = 40f32;
    let screen_bounds = Rect::from([0., 0., window_width, window_height - input_height]);
    let input_bounds = Rect::from([
        0. + 15.,
        screen_bounds.h,
        window_width - 15. * 2.,
        input_height,
    ]);
    (screen_bounds, input_bounds)
}
