//! ブロック崩し本体の実装

/// レンダラ
pub mod renderer;

/// 状態
pub mod state;

/// エンティティ
pub mod entities;

pub struct BreakOut {
    renderer: renderer::BreakOutRenderer, 
    state: state::BreakOutGameState, 
    entities: entities::BreakOutEntities, 
}
impl BreakOut {
    pub fn new(
        gfx_ctx: &crate::gfx::WGContext, 
    ) -> anyhow::Result<Self> {
        let renderer = renderer::BreakOutRenderer::new(gfx_ctx)?;
        let state = state::BreakOutGameState::new();
        let entities = entities::BreakOutEntities::new([
            gfx_ctx.size.width as f32, 
            gfx_ctx.size.height as f32, 
        ].into());
        Ok(Self {
            renderer, 
            state, 
            entities, 
        })
    }
}
impl super::scene::GameScene for BreakOut {
    fn name(&self) -> std::borrow::Cow<'static, str> {
        "ブロック崩し".into()
    }

    fn update(
        &mut self, 
        state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext
    ) -> anyhow::Result<super::scene::SceneController> {
        self.entities.update(
            [
                gfx_ctx.size.width as f32, 
                gfx_ctx.size.height as f32, 
            ].into(), 
            &mut self.state
        );
        self.renderer.update(&self.entities);
        Ok(super::scene::SceneController::NOp)
    }

    fn key_input(
        &mut self, 
        keycode: winit::event::VirtualKeyCode, 
        elem_state: winit::event::ElementState
    ) { 
        self.entities.key_input(keycode, elem_state);
    }

    fn mouse_button_input(&mut self, button: winit::event::MouseButton, elem_state: winit::event::ElementState) {
        
    }

    fn mouse_wheel_input(&mut self, delta: winit::event::MouseScrollDelta) {
        
    }

    fn mouse_motion_input(&mut self, delta: crate::MouseMoveInput) {
        log::debug!("{delta:?}");
        self.entities.mouse_motion_input(delta);
    }

    fn rendering(
        &mut self, 
        state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext, 
        rendering_chain: crate::gfx::RenderingChain
    ) -> crate::gfx::RenderingChain {
        let r: [&mut dyn crate::gfx::WGRenderer; 1] = [
            &mut self.renderer
        ];
        rendering_chain.rendering(gfx_ctx, r)
    }

    
}