//! シーンシステムの実装

use std::{collections::VecDeque, borrow::Cow};
use winit::event::{VirtualKeyCode, ElementState, MouseButton, MouseScrollDelta};

pub trait GameScene {
    fn name(&self) -> Cow<'static, str>;
    fn update(
        &mut self, 
        state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext
    ) -> anyhow::Result<SceneController>;
    fn key_input(&mut self, keycode: VirtualKeyCode, elem_state: ElementState);
    fn mouse_button_input(&mut self, button: MouseButton, elem_state: ElementState);
    fn mouse_wheel_input(&mut self, delta: MouseScrollDelta);
    fn mouse_motion_input(&mut self, delta: crate::MouseMoveInput);
    fn rendering(
        &mut self, 
        state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext, 
        rendering_chain: crate::gfx::RenderingChain
    ) -> crate::gfx::RenderingChain;
}

pub struct SceneCollector (VecDeque<Box<dyn GameScene>>);
impl SceneCollector {
    pub(super) fn new(default_scene: Box<dyn GameScene>) -> Self {
        let mut vd = VecDeque::new();
        vd.push_back(default_scene);
        Self(vd)
    }
    pub fn new_scene(&mut self, scene: Box<dyn GameScene>) {
        self.0.push_back(scene)
    }
    pub fn pop_scene(&mut self) -> Option<Box<dyn GameScene>> {
        self.0.pop_back()
    }
    pub fn key_input(&mut self, keycode: VirtualKeyCode, elem_state: ElementState) {
        self.0.back_mut().map(|s| s.key_input(keycode, elem_state));
    }
    pub fn mouse_button_input(&mut self, button: MouseButton, elem_state: ElementState) {
        self.0.back_mut().map(|s| s.mouse_button_input(button, elem_state));
    }
    pub fn mouse_wheel_input(&mut self, delta: MouseScrollDelta) {
        self.0.back_mut().map(|s| s.mouse_wheel_input(delta));
    }
    pub fn mouse_motion_input(&mut self, delta: crate::MouseMoveInput) {
        self.0.back_mut().map(|s| s.mouse_motion_input(delta));
    }
    pub fn flush_all(&mut self) {
        self.0.clear()
    }
    pub fn update(
        &mut self, 
        state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext
    ) -> anyhow::Result<SceneUpdateResult> {
        match self.0.back_mut() {
            Some(back) => Some(back.update(state, gfx_ctx)?),
            None => None,
        }.map_or_else(
            || Ok(SceneUpdateResult::EmptyScene), 
            |c| match c {
                SceneController::NOp => {
                    Ok(SceneUpdateResult::Updated(None))
                }, 
                SceneController::NewScene(ns) => {
                    log::info!("scene collector pushed new scene.");
                    log::debug!("new scene name: {0}", ns.name());
                    self.0.push_back(ns);
                    Ok(SceneUpdateResult::Updated(None))
                },
                SceneController::RefleshScene(ns) => {
                    log::info!("scene collector refleshed.");
                    log::debug!("new scene name: {0}", ns.name());
                    self.0.clear();
                    self.0.push_back(ns);
                    Ok(SceneUpdateResult::Updated(None))
                },
                SceneController::PopScene => {
                    let bk = self.0.pop_back();
                    log::info!("scene collector back popped.");
                    log::debug!("popped scene: {0:?}", bk.as_ref().map(|b| b.name()));
                    Ok(SceneUpdateResult::Updated(bk))
                },
            }
        )
    }
    pub fn rendering(
        &mut self, 
        state: &mut super::state::GameState, 
        gfx_ctx: &crate::gfx::WGContext, 
        mut rendering_chain: crate::gfx::RenderingChain
    ) {
        for r in self.0.iter_mut() {
            rendering_chain = r.rendering(
                state, 
                gfx_ctx, 
                rendering_chain
            );
        }
        rendering_chain.present();
    }
}

/// シーンの制御データ
pub enum SceneController {
    /// なにもしない
    NOp, 
    /// スタックの末端に新たなシーンをプッシュする
    NewScene(Box<dyn GameScene>), 
    /// スタックに積まれたシーンを除去し、新しいシーンをプッシュする
    RefleshScene(Box<dyn GameScene>), 
    /// スタックの末端のシーンをポップする
    PopScene, 
}

/// シーンの処理結果
pub enum SceneUpdateResult {
    Updated(Option<Box<dyn GameScene>>), 
    EmptyScene, 
}