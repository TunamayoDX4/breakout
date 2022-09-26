//! ブロック崩し本体の実装

use rodio::Source;

/// レンダラ
pub mod obj_renderer;

/// テキストのレンダラ
pub mod text_renderer;

/// 状態
pub mod state;

/// エンティティ
pub mod entities;

pub struct BreakOut<BF: entities::brick::brick::BrickFeature> {
    text: text_renderer::BreakOutGameTextRenderer, 
    renderer: obj_renderer::BreakOutRenderer, 
    state: state::BreakOutGameState, 
    entities: entities::BreakOutEntities<BF>, 
    to_pause: bool, 
}
impl<BF: entities::brick::brick::BrickFeature> BreakOut<BF> {
    pub fn new(
        gfx_ctx: &crate::gfx::WGContext, 
        text_glyph: super::util::text_renderer::TextRendererGMArc, 
        brick_param: entities::brick::BrickSpawnParam<
            impl Into<nalgebra::Vector2<f32>>, 
            impl Into<nalgebra::Vector2<f32>>, 
            impl FnMut(
                [u32; 2], 
                nalgebra::Point2<f32>, 
                nalgebra::Vector2<f32>, 
            ) -> Option<entities::brick::Brick<BF>>, 
            BF
        >
    ) -> anyhow::Result<Self> {
        let renderer = obj_renderer::BreakOutRenderer::new(gfx_ctx)?;
        let state = state::BreakOutGameState::new();
        let entities = entities::BreakOutEntities::new(
            brick_param, 
            [
                gfx_ctx.size.width as f32, 
                gfx_ctx.size.height as f32, 
            ].into()
        );
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
impl<BF: entities::brick::brick::BrickFeature> super::scene::GameScene for BreakOut<BF> {
    fn name(&self) -> std::borrow::Cow<'static, str> {
        "ブロック崩し".into()
    }

    fn update(
        &mut self, 
        state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext, 
        sfx_ctx: &crate::sfx::SfxModule, 
    ) -> anyhow::Result<super::scene::SceneController> {
        self.entities.update(
            [
                gfx_ctx.size.width as f32, 
                gfx_ctx.size.height as f32, 
            ].into(), 
            &mut self.state, 
            sfx_ctx
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
            entry.text_mut()[3].text = format!(" スコア : {0} ", *self.state.score.lock()).into()
        });
        if !self.to_pause {
            Ok(super::scene::SceneController::NOp)
        } else {
            self.to_pause = false;
            sfx_ctx.play_resource("pause", |r| r);
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