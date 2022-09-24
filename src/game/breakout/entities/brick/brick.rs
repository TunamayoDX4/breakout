use super::BBCollisionPoint;

/// ブロック
pub struct Brick {
    model: super::Instance, 
    pub score: u64, 
}
impl Brick {
    pub fn spawn(
        position: nalgebra::Point2<f32>, 
        size: nalgebra::Vector2<f32>, 
        color: [f32; 4], 
        score: u64, 
    ) -> Self { Self { 
        model: super::Instance {
            position,
            size,
            angle: 0.,
            color,
        }, 
        score, 
    } }
    pub fn collision(
        &self, 
        ball: &super::super::ball::Ball
    ) -> Option<BBCollisionPoint> {
        if self.model.collision_aabb(&ball.model) {
            let bb_length = (
                self.model.position - ball.model.position
            ).normalize();
            let aspect = self.model.size.x / self.model.size.y;
            match (
                bb_length.x.abs() / bb_length.y.abs() < aspect, 
                bb_length.x.is_sign_positive(), 
                bb_length.y.is_sign_positive()
            ) {
                (true, true, true) => Some(BBCollisionPoint::Top),
                (true, true, false) => Some(BBCollisionPoint::Bottom),
                (true, false, true) => Some(BBCollisionPoint::Top),
                (true, false, false) => Some(BBCollisionPoint::Bottom),
                (false, true, true) => Some(BBCollisionPoint::Right),
                (false, true, false) => Some(BBCollisionPoint::Right),
                (false, false, true) => Some(BBCollisionPoint::Left),
                (false, false, false) => Some(BBCollisionPoint::Left),
            }
        } else {
            None
        }
    }
    pub fn hit(&self, mut f: impl FnMut(&Self)) { f(self) }
}
impl super::AsInstance for Brick {
    fn as_instance(&self, instances: &mut super::super::RawInstArray) {
        instances.push(&self.model)
    }
}