mod colors;
mod draw;
mod game;
mod physics;
mod snake;

use draw::blocks_in_pixels;
use game::Game;
use piston_window::*;

const WINDOW_TITLE: &'static str = "rsnake";
const WIDTH: u32 = 25;
const HEIGHT: u32 = 25;

fn main() {
    // 计算窗口大小
    let size = [blocks_in_pixels(WIDTH), blocks_in_pixels(HEIGHT)];

    // 创建PistonWindow窗口
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, size)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    // 找到assets游戏资产文件
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    // 从assets游戏资产文件中加载字体
    let ref font = assets.join("retro-gaming.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(
        font,
        TextureContext {
            factory,
            encoder: window.factory.create_command_buffer().into(),
        },
        TextureSettings::new(),
    )
    .unwrap();

    let mut main: Game = Game::new(WIDTH, HEIGHT);
    main.start();

    while let Some(event) = window.next() {
        // 监控按键事件,触发Game.key_down()
        if let Some(Button::Keyboard(key)) = event.press_args() {
            main.key_down(key);
        }

        // 打印2D窗口
        window.draw_2d(&event, |ctx, g, device| {
            clear(colors::BACKGROUND, g);
            // 记录分数
            text::Text::new_color(colors::SCORE, 20)
                .draw(
                    main.get_score().to_string().as_ref(),
                    &mut glyphs,
                    &ctx.draw_state,
                    ctx.transform.trans(0.0, 20.0),
                    g,
                )
                .unwrap();
            glyphs.factory.encoder.flush(device);

            main.draw(ctx, g);
        });

        event.update(|arg| {
            main.update(arg.dt);
        });
    }
}
