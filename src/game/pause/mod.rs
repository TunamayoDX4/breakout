//! ブロック崩しのポーズ画面の実装

mod renderer;

/// ポーズ画面の実装
pub struct Pause {
    renderer: renderer::PauseRenderer, 
    returned: bool, 
}
impl Pause {
    pub fn new(
        text_glyph: super::util::text_renderer::TextRendererGMArc, 
    ) -> anyhow::Result<Self> {
        let renderer = renderer::PauseRenderer::new(
            text_glyph
        )?;
        Ok(Self {
            renderer, 
            returned: false, 
        })
    }
}
impl super::scene::GameScene for Pause {
    fn name(&self) -> std::borrow::Cow<'static, str> {
        "ポーズ画面".into()
    }

    fn update(
        &mut self, 
        _state: &mut super::state::GameState, 
        _gfx_ctx: &crate::gfx::WGContext, 
        sfx_ctx: &crate::sfx::SfxModule, 
    ) -> anyhow::Result<super::scene::SceneController> {
        if self.returned {
            sfx_ctx.play_resource("pause", |r| r);
            Ok(super::scene::SceneController::PopScene)
        } else {
            Ok(super::scene::SceneController::NOp)
        }
    }

    fn key_input(
        &mut self, 
        keycode: winit::event::VirtualKeyCode, 
        elem_state: winit::event::ElementState
    ) {
        match keycode {
            winit::event::VirtualKeyCode::O => {
                self.returned = elem_state == winit::event::ElementState::Pressed;
            }, 
            _ => {}, 
        }
    }

    fn mouse_button_input(
        &mut self, 
        _button: winit::event::MouseButton, 
        _elem_state: winit::event::ElementState
    ) {
    }

    fn mouse_wheel_input(&mut self, _delta: winit::event::MouseScrollDelta) {
    }

    fn mouse_motion_input(&mut self, _delta: crate::MouseMoveInput) {
    }

    fn rendering(
        &mut self, 
        _state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext, 
        rendering_chain: crate::gfx::RenderingChain
    ) -> crate::gfx::RenderingChain {
        let r: [&mut dyn crate::gfx::WGRenderer; 1] = [
            &mut self.renderer, 
        ];
        rendering_chain.rendering(gfx_ctx, r)
    }
}