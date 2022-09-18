//! ゲーム本体の実装

type PMutex<T> = parking_lot::Mutex<T>;

/// ユーティリティ
pub mod util;

/// ゲーム本体の状態
pub mod state;

/// シーンシステム
pub mod scene;

/// ブロック崩しのメインシステム
pub mod breakout;

/// ポーズ画面
pub mod pause;

use winit::event::{VirtualKeyCode, ElementState, MouseButton, MouseScrollDelta};
pub struct GameCtx {
    gfx_ctx: std::sync::Arc<PMutex<crate::gfx::WGContext>>, 
    scenes: scene::SceneCollector, 
    state: state::GameState, 
    exit: bool, 
}
impl GameCtx {
    pub fn new(
        gfx_ctx: std::sync::Arc<PMutex<crate::gfx::WGContext>>, 
        mut default_scene: impl FnMut(
            &crate::gfx::WGContext, 
            &mut state::GameState
        ) -> anyhow::Result<
            Box<dyn scene::GameScene>
        >, 
    ) -> anyhow::Result<Self> { 
        let (scenes, state) = {
            let gfx_ctx_lock = gfx_ctx.lock();
            let mut state = state::GameState::new(
                &gfx_ctx_lock
            )?;
            let scenes = scene::SceneCollector::new(
                default_scene(&gfx_ctx_lock, &mut state)?
            );
            (scenes, state)
        };
        Ok(Self {
            gfx_ctx,
            scenes, 
            state, 
            exit: false, 
        })
    }
    pub fn update(&mut self) -> anyhow::Result<scene::SceneUpdateResult> {
        if self.exit { self.scenes.flush_all() }
        self.scenes.update(
            &mut self.state, 
            &self.gfx_ctx.lock()
        )
    }
    pub fn key_input(&mut self, keycode: VirtualKeyCode, elem_state: ElementState) {
        self.scenes.key_input(match keycode {
            a @ VirtualKeyCode::Escape => {
                self.exit = true;
                a
            }, 
            a @ _ => a, 
        }, elem_state)
    }
    pub fn mouse_button_input(&mut self, button: MouseButton, elem_state: ElementState) {
        self.scenes.mouse_button_input(button, elem_state)
    }
    pub fn mouse_wheel_input(&mut self, delta: MouseScrollDelta) {
        self.scenes.mouse_wheel_input(delta)
    }
    pub fn mouse_motion_input(&mut self, delta: super::MouseMoveInput) {
        self.scenes.mouse_motion_input(delta)
    }
    pub fn rendering(&mut self, rendering_chain: crate::gfx::RenderingChain) {
        self.scenes.rendering(
            &mut self.state, 
            &self.gfx_ctx.lock(), 
            rendering_chain
        )
    }
}