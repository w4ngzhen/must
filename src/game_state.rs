use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawParam};
use telnet::Telnet;
use crate::screen::Screen;
use crate::telnet_client::TelnetData;

pub struct GameState {
    telnet_client: Telnet,
    screen: Screen,
}


impl GameState {
    pub fn new(_ctx: &mut Context) -> Self {
        let size = _ctx.gfx.window().inner_size();
        let mut telnet_client = Telnet::connect(("pkuxkx.net", 8081), 1024).expect("Couldn't connect to the server...");
        Self {
            telnet_client,
            screen: Screen::new(size.width, size.height),
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
        canvas.draw(&self.screen, DrawParam::default());
        // Draw code here...
        canvas.finish(ctx)
    }
}