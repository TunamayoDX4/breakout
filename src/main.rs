use winit::{event_loop::EventLoop, window::WindowBuilder, event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode}};

/// グラフィクス
pub mod gfx;

/// ゲーム
pub mod game;

/// コンテキストのスポーン及び実行
async fn run() -> anyhow::Result<()> {
    let ev_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(winit::dpi::PhysicalSize::new(640, 640))
        .build(&ev_loop)?;
    let mut wgpu_ctx = gfx::WGContext::new(&window).await?;
    let mut game_ctx = game::GameCtx::new(nalgebra::Vector2::new(
        window.inner_size().width as f32, 
        window.inner_size().height as f32
    ));

    // イベントをポーリングします。終了した場合はmainには戻らず、ここで終了となります。
    ev_loop.run(move |ev, _, ctl| match ev {
        Event::WindowEvent { 
            window_id, 
            ref event 
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => ctl.set_exit(), 
            WindowEvent::Resized(new_size) => wgpu_ctx.resize(*new_size), 
            WindowEvent::KeyboardInput { 
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                }, 
                .. 
            } => {
                let is_pressed = *state == winit::event::ElementState::Pressed;
                match keycode {
                    kc @ (
                        VirtualKeyCode::A
                        | VirtualKeyCode::D
                        | VirtualKeyCode::Space
                    ) => game_ctx.input(
                        is_pressed, kc
                    ), 
                    _ => {}, 
                }
            }
            _ => {}, 
        }, 
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            // 再描画処理
            match wgpu_ctx.rendering(&game_ctx) {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => wgpu_ctx.re_configure(),
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    log::error!("重大なエラー: レンダリングに必要なVRAM領域が不足しています。");
                    ctl.set_exit();
                }, 
                Err(e) => {
                    log::error!("描画処理中にエラーが発生しました。内容は以下の通りです。");
                    eprintln!("{e:?}");
                }, 
            }
        }, 
        Event::MainEventsCleared => {
            // 全てのイベントがクリアされたらゲームの処理及び再描画
            let size = wgpu_ctx.size();
            let size = nalgebra::Vector2::<f32>::new(size.width as f32, size.height as f32);
            game_ctx.update(size);
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