use ggez::{ContextBuilder, event, graphics};
use ggez::conf::{WindowMode, WindowSetup};
use crate::constants::FONT_FLAG_NAME;
use crate::game_state::GameState;

mod game_state;
mod screen;
mod constants;
mod ui;
mod utils;

/// 开发运行前，请确保编译目录（理论上为target目录）下debug目录中，must可执行程序同级下有resources目录，
/// 且目录中有字体文件："DejaVuSansMono YaHei NF"
/// 该字体文件请从项目目录/resources/目录下复制到对应目录
fn main() {
    // 建立游戏客户端
    let (mut ctx, event_loop) =
        ContextBuilder::new("must", "w4nzhen")
            .window_mode(WindowMode::default().dimensions(1200., 800.).resizable(true))
            .window_setup(WindowSetup::default().title("MUST: mud client by rust"))
            .build()
            .expect("Could not create ggez context!");
    ctx.gfx.add_font(
        FONT_FLAG_NAME,
        graphics::FontData::from_path(&ctx, "/DejaVuSansMono YaHei NF.ttf").expect("load font error"),
    );
    let my_game = GameState::new(&mut ctx);
    // Run!
    event::run(ctx, event_loop, my_game);
}
