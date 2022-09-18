//! ブロック崩しのエンティティの実装

use winit::event::{VirtualKeyCode, ElementState, MouseButton};

use super::obj_renderer::model::{Instance, AsInstance, RawInstArray};

pub mod brick;
pub mod ball;
pub mod paddle;
pub mod pointer;

pub struct BreakOutEntities {
    bricks: brick::BrickColumn, 
    ball: Option<ball::Ball>, 
    paddle: paddle::Paddle, 
    pointer: pointer::Pointer, 
}
impl BreakOutEntities {
    pub fn new(
        disp_size: nalgebra::Vector2<f32>, 
    )-> Self { Self {
        paddle: paddle::Paddle::spawn(
            [disp_size.x / 2., 120.].into(), 
            [1., 1., 1., 1.]
        ),
        ball: None,
        pointer: pointer::Pointer::spawn(), 
        bricks: brick::BrickColumn::spawn(
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
            b.refle_brick(&mut self.bricks);
            b.moving(state);
            b.update(state);
            b.despawnable()
        }) { b } else { false } { 
            state.remain_ball -= 1;
            self.ball = None 
        }
        if self.bricks.count() == 0 { state.state = super::state::GameState::GameClear }
        self.paddle.update(disp_size, state, &mut self.ball);
        self.paddle.change_color(state, &self.ball);
    }
    pub fn key_input(&mut self, keycode: VirtualKeyCode, state: ElementState) {
        let state = state == ElementState::Pressed;
        match keycode {
            VirtualKeyCode::A => self.paddle.move_flag.move_left = state, 
            VirtualKeyCode::D => self.paddle.move_flag.move_right = state, 
            VirtualKeyCode::Space => self.paddle.move_flag.ball_shot = state, 
            _ => {}, 
        }
    }
    pub fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        let state = state == ElementState::Pressed;
        match button {
            MouseButton::Left => self.paddle.move_flag.ball_shot = state,
            _ => {}, 
        }
    }
    pub fn mouse_motion_input(&mut self, input: crate::MouseMoveInput) {
        self.paddle.move_flag.move_delta = input.0.x;
    }
    pub fn remain_brick(&self) -> usize {
        self.bricks.count()
    }
}
impl AsInstance for BreakOutEntities {
    fn as_instance(&self, instances: &mut super::obj_renderer::model::RawInstArray) {
        self.paddle.as_instance(instances);
        self.bricks.as_instance(instances);
        if let Some(ball) = self.ball.as_ref() { ball.as_instance(instances) };
        self.pointer.as_instance(instances);
    }
}