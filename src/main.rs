use std::sync::Arc;

use parking_lot::Mutex;
use winit::{
    event_loop::EventLoop, 
    window::WindowBuilder, 
    event::{
        Event, 
        WindowEvent, 
        KeyboardInput, 
        DeviceEvent
    }
};

/// グラフィクス
pub mod gfx;

/// サウンドエフェクト
pub mod sfx;

/// ゲーム本体の実装
pub mod game;

/// マウスの移動率入力のバッファ
pub struct MouseMoveBuffer {
    move_vol: nalgebra::Vector2<f32>, 
}
impl MouseMoveBuffer {
    pub fn new() -> Self { Self {
        move_vol: Default::default()
    } }
    pub fn input(&mut self, move_vol: nalgebra::Vector2<f32>) {
        self.move_vol += move_vol
    }
    pub fn finalize(&mut self) -> MouseMoveInput {
        let input = MouseMoveInput(self.move_vol);
        self.move_vol = Default::default();
        input
    }
}

/// マウスの移動率データ
#[derive(Clone, Copy, Debug)]
pub struct MouseMoveInput(pub nalgebra::Vector2<f32>);

/// コンテキストのスポーン及び実行
async fn run() -> anyhow::Result<()> {
    let ev_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(winit::dpi::PhysicalSize::new(640, 640))
        .with_title("BreakOut ～ブロック崩し～")
        .build(&ev_loop)?;
    let wgpu_ctx = Arc::new(
        parking_lot::Mutex::new(gfx::WGContext::new(&window).await?)
    );

    let mut sfx_ctx = sfx::SfxModule::new(0.25)?;
    sfx_ctx.add_resource(
        "pause", 
        rodio::Decoder::new_mp3(
            std::fs::File::open("./se/pause.mp3")?
        )?
    );
    sfx_ctx.add_resource(
        "break", 
        rodio::Decoder::new_mp3(
            std::fs::File::open("./se/break.mp3")?
        )?
    );
    sfx_ctx.add_resource(
        "miss", 
        rodio::Decoder::new_mp3(
            std::fs::File::open("./se/miss.mp3")?
        )?    
    );
    sfx_ctx.add_resource(
        "reflection", 
        rodio::Decoder::new_mp3(
            std::fs::File::open("./se/reflection.mp3")?
        )?    
    );

    let mut game_ctx = game::GameCtx::new(
        Arc::clone(&wgpu_ctx), 
        |ctx, state| {
            Ok(Box::new(game::breakout::BreakOut::new(
                ctx, 
                state.ipaexg.clone(), 
                game::breakout::entities::brick::BrickSpawnParam {
                    column: 32,
                    row: 56,
                    margin_top: 32.,
                    brick_margin: [2., 2.],
                    brick_size: [8., 4.],
                    spawn_f: Arc::new(Mutex::new(|
                        pos: [u32; 2], 
                        blk_pos, 
                        blk_size, 
                    | {
                        if pos[1] % 3 == 0 {
                            None
                        } else {
                            Some(game::breakout::entities::brick::Brick::spawn(
                                blk_pos, 
                                blk_size, 
                                [
                                    1. - pos[1] as f32 * (1. / 18.), 
                                    pos[0] as f32 * (1. / 18.), 
                                    pos[1] as f32 * (1. / 18.), 
                                    1.
                                ], 
                                100 * pos[1] as u64, 
                            ))
                        }
                    })),
                }, 
            )?))
        }
    )?;
    let mut mouse_buffer = MouseMoveBuffer::new();


    // イベントをポーリングします。終了した場合はmainには戻らず、ここで終了となります。
    ev_loop.run(move |ev, _, ctl| match ev {
        Event::WindowEvent { 
            window_id, 
            ref event 
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => ctl.set_exit(), 
            WindowEvent::Resized(new_size) => wgpu_ctx.lock().resize(*new_size), 
            WindowEvent::KeyboardInput { 
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                }, 
                .. 
            } => game_ctx.key_input(*keycode, *state), 
            WindowEvent::MouseInput { 
                state, 
                button, 
                .. 
            } => game_ctx.mouse_button_input(*button, *state), 
            WindowEvent::MouseWheel { 
                delta, 
                .. 
            } => game_ctx.mouse_wheel_input(*delta), 
            _ => {}, 
        }, 
        Event::DeviceEvent { 
            event: DeviceEvent::MouseMotion { delta }, 
            ..
        } => mouse_buffer.input([delta.0 as f32, delta.1 as f32].into()), 
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            // 再描画処理
            let rc = match wgpu_ctx.lock().rendering() {
                Ok(rc) => Some(rc),
                Err(wgpu::SurfaceError::Lost) => {
                    wgpu_ctx.lock().re_configure();
                    None
                },
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    log::error!("重大なエラー: レンダリングに必要なVRAM領域が不足しています。");
                    ctl.set_exit();
                    None
                }, 
                Err(e) => {
                    log::error!("描画処理中にエラーが発生しました。内容は以下の通りです。");
                    eprintln!("{e:?}");
                    None
                }, 
            };
            if let Some(rc) = rc { game_ctx.rendering(rc) }
        }, 
        // すべてのイベントがクリアされたら
        Event::MainEventsCleared => {
            window.set_cursor_visible(false);
            window.set_cursor_grab(
                winit::window::CursorGrabMode::Confined
            ).expect("Set cursor grab error");
            // マウス入力の完了
            let input = mouse_buffer.finalize();
            game_ctx.mouse_motion_input(input);

            // ゲームの処理
            match game_ctx.update(&sfx_ctx).expect("Game context update error") {
                game::scene::SceneUpdateResult::Updated(_) => {},
                game::scene::SceneUpdateResult::EmptyScene => ctl.set_exit(),
            };

            // 再描画
            window.request_redraw();
        }, 
        _ => {},  
    });
}

/// ロガーの初期化
fn fern_init() -> anyhow::Result<()> {
    fern::Dispatch::new()
        // 出力フォーマットの指定
        .format(|
                out, 
                msg, 
                rec
            | out.finish(format_args!(
            // [13:51:23][tunamayo-shooting][INFO]: サンプルログ
            "{0}[{1}][{2}]: {3}", 
            chrono::Local::now().format("[%H:%M:%S]"), 
            rec.target(), 
            rec.level(), 
            msg, 
        )))
        // ログの表示レベルの指定
        // INFO以下の優先度のログは破棄
        .level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .level_for(
            "wgpu_core::device", 
            if cfg!(debug_assertions) {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Warn
            }
        )
        // ログのアウトプット先の指定
        // 標準出力を指定
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

/// エントリポイント
fn main() -> anyhow::Result<()> {
    // ログの初期化
    fern_init()?;

    // コンテキストの実行
    pollster::block_on(run())
}