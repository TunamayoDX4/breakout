use super::BBCollisionPoint;

/// ブロックの機能実装用トレイト
pub trait BrickFeature {
    fn hitted_process(
        &self, 
        state: &mut super::super::super::state::BreakOutGameState, 
    );
}

/// ブロック
pub struct Brick<BF: BrickFeature> {
    feature: BF, 
    model: super::Instance, 
}
impl<BF: BrickFeature> Brick<BF> {
    pub fn spawn(
        feature: BF, 
        position: nalgebra::Point2<f32>, 
        size: nalgebra::Vector2<f32>, 
        color: [f32; 4], 
    ) -> Self { Self { 
        feature, 
        model: super::Instance {
            position,
            size,
            angle: 0.,
            color,
        }, 
    } }
    pub fn collision(
        &self, 
        ball: &super::super::ball::Ball, 
        _state: &mut super::super::super::state::BreakOutGameState
    ) -> Option<BBCollisionPoint> {
        let ball_delta = [
            ball.model.position, 
            ball.model.position + ball.angle * ball.speed, 
        ];

        {
            let mut coll_delta = None;
            let (a, b) = (
                &ball_delta[0], 
                &ball_delta[1], 
            );
            
            let mut length = 1.0;

            for (
                count, brick_delta
            ) in self.model.edges().into_iter()
                .enumerate()
            {
                let (c, d) = (
                    &brick_delta[0], 
                    &brick_delta[1], 
                );
                let v_ab = b - a;
                let v_cd = d - c;
                let v_ac = c - a;

                let bunbo = (v_ab.x * v_cd.y) - (v_ab.y * v_cd.x);
                if bunbo.abs() <= f32::EPSILON { continue; }
                let r = (v_cd.y * v_ac.x - v_cd.x * v_ac.y) / bunbo;
                let s = (v_ab.y * v_ac.x - v_ab.x * v_ac.y) / bunbo;

                if 0. <= r && r <= 1. && 0. <= s && s <= 1. {
                    if r < length {
                        length = r;
                        coll_delta = Some(match count {
                            0 => BBCollisionPoint::Bottom, 
                            1 => BBCollisionPoint::Top, 
                            2 => BBCollisionPoint::Left, 
                            _ => BBCollisionPoint::Right, 
                        });
                    }
                }}

            coll_delta
        }
    }
    pub fn hit(&self, mut f: impl FnMut(&Self)) { f(self) }
}
impl<BF: BrickFeature> super::AsInstance for Brick<BF> {
    fn as_instance(&self, instances: &mut super::super::RawInstArray) {
        instances.push(&self.model)
    }
}