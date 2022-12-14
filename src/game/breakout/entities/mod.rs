//! ブロック崩しのエンティティの実装

use winit::event::{VirtualKeyCode, ElementState, MouseButton};

use super::obj_renderer::model::{Instance, AsInstance, RawInstArray};

pub mod brick;
pub mod ball;
pub mod paddle;
pub mod pointer;

pub struct BreakOutEntities<BF: brick::brick::BrickFeature> {
    bricks: brick::BrickCollection<BF>, 
    ball: Option<ball::Ball>, 
    paddle: paddle::Paddle, 
    pointer: pointer::Pointer, 
}
impl<BF: brick::brick::BrickFeature> BreakOutEntities<BF> {
    pub fn new<BM, BS, SF>(
        brick_param: brick::BrickSpawnParam<BM, BS, SF, BF>, 
        disp_size: nalgebra::Vector2<f32>, 
    )-> Self where
        BM: Into<nalgebra::Vector2<f32>>, 
        BS: Into<nalgebra::Vector2<f32>>, 
        SF: FnMut(
            [u32; 2], 
            nalgebra::Point2<f32>, 
            nalgebra::Vector2<f32>
        ) -> Option<brick::Brick<BF>>, 
    { Self {
        paddle: paddle::Paddle::spawn(
            [disp_size.x / 2., 120.].into(), 
            [1., 1., 1., 1.]
        ),
        ball: None,
        pointer: pointer::Pointer::spawn(), 
        bricks: brick::BrickCollection::spawn(disp_size, brick_param), 
    }}
    pub fn update(
        &mut self, 
        disp_size: nalgebra::Vector2<f32>, 
        state: &mut super::state::BreakOutGameState, 
        sfx_ctx: &crate::sfx::SfxModule, 
    ) {
        if if let Some(b) = self.ball.as_mut().map(|b| {
            b.refle_edge(disp_size, sfx_ctx);
            b.refle_paddle(&self.paddle, &mut self.pointer, sfx_ctx);
            b.refle_brick(
                self.bricks.get_mut(), 
                state, 
                sfx_ctx, 
            );
            b.moving(state);
            b.update(state, &state.difficulity);
            b.despawnable(sfx_ctx)
        }) { b } else { false } { 
            state.remain_ball -= 1;
            self.ball = None 
        }
        if self.bricks.get().count() == 0 { state.state = super::state::GameState::GameClear }
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
        self.bricks.get().count()
    }
}
impl<BF: brick::brick::BrickFeature> AsInstance for BreakOutEntities<BF> {
    fn as_instance(&self, instances: &mut super::obj_renderer::model::RawInstArray) {
        self.paddle.as_instance(instances);
        self.bricks.as_instance(instances);
        if let Some(ball) = self.ball.as_ref() { ball.as_instance(instances) };
        self.pointer.as_instance(instances);
    }
}