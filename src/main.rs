use std::sync::mpsc::channel;
use std::thread;
use ggez::{ContextBuilder, event};
use crate::game_state::GameState;
use crate::telnet_client::{TelnetClient, TelnetData};

mod game_state;
mod telnet_client;

fn main() {
    // 建立线程通道
    let (sender, receiver) = channel::<TelnetData>();
    // 建立Telnet客户端
    let mut client = TelnetClient::new(sender);
    thread::spawn(move || {
        client.run();
    });
    // 建立游戏客户端
    let (mut ctx, event_loop) = ContextBuilder::new("must", "w4nzhen")
        .build()
        .expect("Could not create ggez context!");
    let my_game = GameState::new(&mut ctx, receiver);
    // Run!
    event::run(ctx, event_loop, my_game);
}
