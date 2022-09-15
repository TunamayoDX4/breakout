//! ゲームの実装

use std::sync::{Arc, Weak};

use winit::event::VirtualKeyCode;

use crate::gfx::model::{AsInstance, Instance};

/// 状態
pub mod state;

/// エンティティ
pub mod entities;

/// ゲームのコンテキスト
pub struct GameCtx {
    state: state::BreakOutGameState, 
    entities: entities::BreakOutEntities, 
}
impl GameCtx {
    pub fn new(disp_size: nalgebra::Vector2<f32>) -> Self {
        let state = state::BreakOutGameState::new();
        let entities = entities::BreakOutEntities::new(
            disp_size, 
        );
        Self {
            state, 
            entities, 
        }
    }
    pub fn update(&mut self, disp_size: nalgebra::Vector2<f32>) {
        self.entities.update(disp_size, &mut self.state);
    }
    pub fn input(&mut self, press: bool, key: &VirtualKeyCode) {
        self.entities.input(press, key);
    }
}
impl AsInstance for GameCtx {
    fn as_instance(&self, instances: &mut crate::gfx::model::RawInstArray) {
        self.entities.as_instance(instances);
    }
}