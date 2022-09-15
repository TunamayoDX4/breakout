/// パドルの移動フラグ
pub struct PaddleMoveFlag {
    pub move_left: bool, 
    pub move_right: bool, 
    pub ball_shot: bool, 
}
impl Default for PaddleMoveFlag {
    fn default() -> Self { Self { 
        move_left: false, 
        move_right: false, 
        ball_shot: false, 
    }}
}

/// 難易度
#[derive(Default)]
pub enum PaddleDifficulity {
    Easy, 
    #[default]
    Normal, 
    Hard, 
}
impl PaddleDifficulity {
    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        Self::Easy => [128., 8.].into(),
        Self::Normal => [64., 8.].into(),
        Self::Hard => [32., 8.].into(),
    }}
}

/// パドル
pub struct Paddle {
    pub(super) model: super::Instance, 
    pub move_flag: PaddleMoveFlag, 
    difficulity: PaddleDifficulity, 
}
impl Paddle {
    pub fn spawn(
        position: nalgebra::Point2<f32>, 
        color: [f32; 4], 
    ) -> Self { Self {
        model: super::Instance {
            position,
            size: PaddleDifficulity::default().size(),
            angle: 0.,
            color,
        },
        move_flag: Default::default(),
        difficulity: Default::default(),
    } }
    pub fn update(
        &mut self, 
        disp_size: nalgebra::Vector2<f32>, 
        state: &mut super::super::state::BreakOutGameState, 
        ball: &mut Option<super::ball::Ball>, 
    ) {
        self.model.size = self.difficulity.size();
        if self.move_flag.ball_shot && ball.is_none() && state.remain_ball != 0 {
            *ball = Some(super::ball::Ball::spawn(
                [self.model.position.x, self.model.position.y + 8.].into(), 
                [1., 1., 1., 1.], 
                [0., 1.].into(), 
                256. / 60.
            ));
        } else if ball.is_none() {
            if state.remain_ball == 0 { state.state = super::super::state::GameState::GameOver };
        }
        let speed = 256. / 60.;
        if self.move_flag.move_right && self.model.position.x + speed < disp_size.x {
            self.model.position.x += speed;
        }
        if self.move_flag.move_left && 0. < self.model.position.x - speed {
            self.model.position.x -= speed;
        }
    }
    pub fn change_color(
        &mut self, 
        state: &super::super::state::BreakOutGameState, 
        ball: &Option<super::ball::Ball>, 
    ) {
        self.model.color = if ball.is_none() && state.state != super::super::state::GameState::GameOver {
            [1., 0., 0., 1.]
        } else { match state.state {
            crate::game::state::GameState::Yes => [1., 1., 1., 1.],
            crate::game::state::GameState::GameOver => [0., 0., 0., 0.],
            crate::game::state::GameState::GameClear => [0., 0., 1., 0.],
        }}
    }
}
impl super::AsInstance for Paddle {
    fn as_instance(&self, instances: &mut crate::gfx::model::RawInstArray) {
        instances.push(&self.model)
    }
}