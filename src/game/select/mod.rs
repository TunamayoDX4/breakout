//! ステージセレクト画面

pub struct StageSelect {
}
impl super::scene::GameScene for StageSelect {
    fn name(&self) -> std::borrow::Cow<'static, str> {
        "ステージ選択画面".into()
    }

    fn update(
        &mut self, 
        state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext, 
        sfx_ctx: &crate::sfx::SfxModule, 
    ) -> anyhow::Result<super::scene::SceneController> {
        todo!()
    }

    fn key_input(
        &mut self, 
        keycode: winit::event::VirtualKeyCode, 
        elem_state: winit::event::ElementState
    ) {
        todo!()
    }

    fn mouse_button_input(
        &mut self, 
        button: winit::event::MouseButton, 
        elem_state: winit::event::ElementState
    ) {
    }

    fn mouse_wheel_input(&mut self, delta: winit::event::MouseScrollDelta) {
    }

    fn mouse_motion_input(&mut self, delta: crate::MouseMoveInput) {
    }

    fn rendering(
        &mut self, 
        state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext, 
        rendering_chain: crate::gfx::RenderingChain
    ) -> crate::gfx::RenderingChain {
        todo!()
    }
}