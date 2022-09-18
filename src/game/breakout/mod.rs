//! ブロック崩し本体の実装

/// レンダラ
pub mod obj_renderer;

/// テキストのレンダラ
pub mod text_renderer;

/// 状態
pub mod state;

/// エンティティ
pub mod entities;

pub struct BreakOut {
    text: text_renderer::BreakOutGameTextRenderer, 
    renderer: obj_renderer::BreakOutRenderer, 
    state: state::BreakOutGameState, 
    entities: entities::BreakOutEntities, 
    to_pause: bool, 
}
impl BreakOut {
    pub fn new(
        gfx_ctx: &crate::gfx::WGContext, 
        text_glyph: super::util::text_renderer::TextRendererGMArc, 
    ) -> anyhow::Result<Self> {
        let renderer = obj_renderer::BreakOutRenderer::new(gfx_ctx)?;
        let state = state::BreakOutGameState::new();
        let entities = entities::BreakOutEntities::new([
            gfx_ctx.size.width as f32, 
            gfx_ctx.size.height as f32, 
        ].into());
        let text = text_renderer::BreakOutGameTextRenderer::new(
            text_glyph
        )?;
        Ok(Self {
            renderer, 
            state, 
            entities, 
            text, 
            to_pause: false, 
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
        self.text.entry_mut("top").map(|entry| {
            entry.text_mut()[1].text = match self.entities.remain_brick() {
                0 => " ゲームクリア！ ".into(), 
                remain @ _ => format!(" 残りのブロック: {remain} ").into(), 
            };
            entry.text_mut()[2].text = match self.state.remain_ball {
                0 => " ゲームオーバー ".into(), 
                remain @ _ => format!(" 残弾数 : {remain} ").into(), 
            };
        });
        if !self.to_pause {
            Ok(super::scene::SceneController::NOp)
        } else {
            self.to_pause = false;
            Ok(super::scene::SceneController::NewScene(
                Box::new(super::pause::Pause::new(
                    state.ipaexg.clone()
                )?)
            ))
        }
    }

    fn key_input(
        &mut self, 
        keycode: winit::event::VirtualKeyCode, 
        elem_state: winit::event::ElementState
    ) { 
        self.entities.key_input(keycode, elem_state);
        if keycode == winit::event::VirtualKeyCode::P {
            self.to_pause = elem_state == winit::event::ElementState::Pressed;
        }
    }

    fn mouse_button_input(&mut self, button: winit::event::MouseButton, elem_state: winit::event::ElementState) {
        self.entities.mouse_input(button, elem_state);
    }

    fn mouse_wheel_input(&mut self, _delta: winit::event::MouseScrollDelta) {
    }

    fn mouse_motion_input(&mut self, delta: crate::MouseMoveInput) {
        self.entities.mouse_motion_input(delta);
    }

    fn rendering(
        &mut self, 
        _state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext, 
        rendering_chain: crate::gfx::RenderingChain
    ) -> crate::gfx::RenderingChain {
        let r: [&mut dyn crate::gfx::WGRenderer; 2] = [
            &mut self.renderer, 
            &mut self.text, 
        ];
        rendering_chain.rendering(gfx_ctx, r)
    }

    
}