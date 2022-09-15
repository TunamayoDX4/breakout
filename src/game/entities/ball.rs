/// ボール
pub struct Ball {
    pub(super) model: super::Instance, 
    angle: nalgebra::Vector2<f32>, 
    speed: f32, 
}
impl Ball {
    pub fn spawn(
        position: nalgebra::Point2<f32>, 
        color: [f32; 4], 
        angle: nalgebra::Vector2<f32>, 
        speed: f32, 
    ) -> Self { Self {
        model: super::Instance {
            position,
            size: nalgebra::Vector2::new(6., 6.),
            angle: 0.,
            color,
        },
        angle,
        speed,
    } }
    pub fn update(&mut self, state: &super::super::state::BreakOutGameState) {
        match state.state {
            crate::game::state::GameState::Yes => {},
            crate::game::state::GameState::GameOver => self.model.color = [1., 0., 0., 0.],
            crate::game::state::GameState::GameClear => self.model.color = [0., 0., 0., 0.],
        }
    }
    pub fn moving(&mut self, state: &super::super::state::BreakOutGameState) {
        match state.state {
            crate::game::state::GameState::Yes => {
                self.angle = self.angle.normalize();
                self.model.position += self.angle * self.speed;
            },
            _ => {}
        }
    }
    pub fn refle_edge(&mut self, disp_size: nalgebra::Vector2<f32>) {
        let wvx = if disp_size.x <= self.model.position.x {
            Some(nalgebra::Vector2::new(-1., 0.))
        } else if self.model.position.x <= 0. {
            Some(nalgebra::Vector2::new(1., 0.))
        } else {
            None
        };
        let wvy = if disp_size.y <= self.model.position.y {
            Some(nalgebra::Vector2::new(0., -1.))
        } else {
            None
        };
        let wv = match (wvx, wvy) {
            (None, None) => return,
            (None, Some(b)) => b,
            (Some(a), None) => a,
            (Some(a), Some(b)) => a + b,
        };
        let d = -self.angle.dot(&wv);
        self.angle += (d * wv) * 2.;
    }
    pub fn refle_paddle(
        &mut self, 
        paddle: &super::paddle::Paddle, 
        pointer: &mut super::pointer::Pointer, 
    ) {
        let a = &self.model.position;
        let b = self.model.position + self.angle * self.speed;
        let paddle_half_width = nalgebra::Vector2::new(paddle.model.size.x / 2., 0.);
        let c = paddle.model.position - paddle_half_width;
        let d = paddle.model.position + paddle_half_width;

        let ac = c - a;
        let bb = (b.x - a.x) * (d.y - c.y) - (b.y - a.y) * (d.x - c.x);

        // BBがほぼゼロの場合はパドルに当たらない
        if bb.abs() <= std::f32::EPSILON { return }

        let r = ((d.y - c.y) * ac.x - (d.x - c.x) * ac.y) / bb;
        let s = ((b.y - a.y) * ac.x - (b.x - a.x) * ac.y) / bb;

        // ポインターの表示
        if 0. <= s && s <= 1. && 0. <= r {
            let cd = (d - c) * s;
            pointer.model.position = c + cd;
            pointer.visible = true;
        } else {
            pointer.visible = false;
        }

        // 接触
        if 0. <= r && r <= 1. && 0. <= s && s <= 1. {
            let s = s - 0.5;
            let cd = d - c;

            self.model.position = self.model.position + (b - a) * r;

            // 反射ベクトル
            let nv = nalgebra::Vector2::new(
                cd.y, 
                cd.x
            )
                .normalize();
            let is_top = self.angle.dot(&nv).is_sign_negative();
            let nv = nalgebra::Vector2::new(
                nv.x + s * 4., 
                if is_top { nv.y } else { -nv.y }, 
            )
                .normalize();
            self.angle = nv;
        }
    }
    pub fn refle_brick(&mut self, brick: &mut super::brick::BrickColumn) {
        let rv = brick.collision(self)
            .map(|r| match r {
                super::brick::BBCollisionPoint::Top => nalgebra::Vector2::new(0., 1.),
                super::brick::BBCollisionPoint::Bottom => nalgebra::Vector2::new(0., -1.),
                super::brick::BBCollisionPoint::Left => nalgebra::Vector2::new(-1., 0.),
                super::brick::BBCollisionPoint::Right => nalgebra::Vector2::new(1., 0.),
            });
        if let Some(rv) = rv {
            let d = -self.angle.dot(&rv);
            self.angle += (d * rv) * 2.;
        }
    }
    pub fn despawnable(&mut self) -> bool { self.model.position.y.is_sign_negative() }
}
impl super::AsInstance for Ball {
    fn as_instance(&self, instances: &mut super::RawInstArray) {
        instances.push(&self.model)
    }
}