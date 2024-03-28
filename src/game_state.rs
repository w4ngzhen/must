use std::io::{stdout, Write};
use std::sync::mpsc::Receiver;
use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::graphics::Color;
use vte::Parser;
use crate::telnet_client::TelnetData;

pub struct GameState {
    vte_parser: Parser,
    data_receiver: Receiver<TelnetData>,
}


impl GameState {
    pub fn new(_ctx: &mut Context, data_receiver: Receiver<TelnetData>) -> Self {
        Self {
            vte_parser: Parser::new(),
            data_receiver,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if let Ok(telnet_data) = self.data_receiver.try_recv() {
            stdout().write(&telnet_data.buf).expect("error");
        }
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(ctx)
    }
}