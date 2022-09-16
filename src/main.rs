use std::sync::Arc;

use winit::{event_loop::EventLoop, window::WindowBuilder, event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode}};

/// グラフィクス
pub mod gfx;

/// ゲーム本体の実装
pub mod game;

/// コンテキストのスポーン及び実行
async fn run() -> anyhow::Result<()> {
    let ev_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(winit::dpi::PhysicalSize::new(640, 640))
        .build(&ev_loop)?;
    let wgpu_ctx = Arc::new(
        parking_lot::Mutex::new(gfx::WGContext::new(&window).await?)
    );
    let mut game_ctx = game::GameCtx::new(
        Arc::clone(&wgpu_ctx), 
        |ctx| {
            Ok(Box::new(game::breakout::BreakOut::new(ctx)?))
        }
    )?;

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
            _ => {}, 
        }, 
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
        Event::MainEventsCleared => {
            // 全てのイベントがクリアされたらゲームの処理及び再描画
            match game_ctx.update().expect("Game context update error") {
                game::scene::SceneUpdateResult::Updated(_) => {},
                game::scene::SceneUpdateResult::EmptyScene => ctl.set_exit(),
            }
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
        .level(log::LevelFilter::Info)
        // wgpu_core::deviceからのログレベルをWARNに指定
        .level_for("wgpu_core::device", log::LevelFilter::Warn)
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