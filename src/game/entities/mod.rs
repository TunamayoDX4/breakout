//! ゲーム内部のエンティティの実装

use winit::event::VirtualKeyCode;

use crate::gfx::model::{Instance, RawInstArray, AsInstance};

/// パドル
pub mod paddle;

/// ボール
pub mod ball;

/// 着弾地点の表示
pub mod pointer;

/// ブロック
pub mod brick;

/// インジケータ
pub mod indicator;

/// エンティティ
pub struct BreakOutEntities {
    paddle: paddle::Paddle, 
    ball: Option<ball::Ball>, 
    pointer: pointer::Pointer, 
    brick: brick::BrickColumn, 
}
impl BreakOutEntities {
    pub fn new(
        disp_size: nalgebra::Vector2<f32>, 
    ) -> Self { Self {
        paddle: paddle::Paddle::spawn(
            [disp_size.x / 2., 120.].into(), 
            [1., 1., 1., 1.]
        ),
        ball: None,
        pointer: pointer::Pointer::spawn(), 
        brick: brick::BrickColumn::spawn(
            6, 
            18, 
            64., 
            [2., 16.].into(), 
            [32., 16.].into(), 
            disp_size, 
            std::sync::Arc::new(
                parking_lot::Mutex::new(brick::brick_spawner)
            )
        )
    }}
    pub fn update(
        &mut self, 
        disp_size: nalgebra::Vector2<f32>, 
        state: &mut super::state::BreakOutGameState, 
    ) {
        if if let Some(b) = self.ball.as_mut().map(|b| {
            b.refle_edge(disp_size);
            b.refle_paddle(&self.paddle, &mut self.pointer);
            b.refle_brick(&mut self.brick);
            b.moving(state);
            b.update(state);
            b.despawnable()
        }) { b } else { false } { 
            state.remain_ball -= 1;
            self.ball = None 
        }
        if self.brick.count() == 0 { state.state = super::state::GameState::GameClear }
        self.paddle.update(disp_size, state, &mut self.ball);
        self.paddle.change_color(state, &self.ball);
    }
    pub fn input(&mut self, press: bool, key: &VirtualKeyCode) { match key {
        VirtualKeyCode::A => self.paddle.move_flag.move_left = press, 
        VirtualKeyCode::D => self.paddle.move_flag.move_right = press, 
        VirtualKeyCode::Space => self.paddle.move_flag.ball_shot = press, 
        _ => {},    
    }}
}
impl AsInstance for BreakOutEntities {
    fn as_instance(&self, instances: &mut RawInstArray) {
        self.paddle.as_instance(instances);
        if let Some(ball) = &self.ball { ball.as_instance(instances) };
        self.pointer.as_instance(instances);
        self.brick.as_instance(instances);
    }
}