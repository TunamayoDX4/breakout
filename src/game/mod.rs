//! ゲーム本体の実装

type PMutex<T> = parking_lot::Mutex<T>;

/// ゲーム本体の状態
pub mod state;

/// シーンシステム
pub mod scene;

/// ブロック崩しのメインシステム
pub mod breakout;

use winit::event::{VirtualKeyCode, ElementState};
pub struct GameCtx {
    gfx_ctx: std::sync::Arc<PMutex<crate::gfx::WGContext>>, 
    scenes: scene::SceneCollector, 
    state: state::GameState, 
}
impl GameCtx {
    pub fn new(
        gfx_ctx: std::sync::Arc<PMutex<crate::gfx::WGContext>>, 
        mut default_scene: impl FnMut(& crate::gfx::WGContext) -> anyhow::Result<
            Box<dyn scene::GameScene>
        >, 
    ) -> anyhow::Result<Self> { 
        let (scenes, state) = {
            let gfx_ctx_lock = gfx_ctx.lock();
            let scenes = scene::SceneCollector::new(
                default_scene(&gfx_ctx_lock)?
            );
            let state = state::GameState::new(
                &gfx_ctx_lock
            );
            (scenes, state)
        };
        Ok(Self {
            gfx_ctx,
            scenes, 
            state, 
        })
    }
    pub fn update(&mut self) -> anyhow::Result<scene::SceneUpdateResult> {
        self.scenes.update(
            &mut self.state, 
            &self.gfx_ctx.lock()
        )
    }
    pub fn key_input(&mut self, keycode: VirtualKeyCode, elem_state: ElementState) {
        self.scenes.key_input(keycode, elem_state)
    }
    pub fn rendering(&mut self, rendering_chain: crate::gfx::RenderingChain) {
        self.scenes.rendering(
            &mut self.state, 
            &self.gfx_ctx.lock(), 
            rendering_chain
        )
    }
}