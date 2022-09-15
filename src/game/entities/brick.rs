type PMutex<T> = parking_lot::Mutex<T>;
use std::sync::Arc;

/// ブロックとボールのだいたいの接触位置
pub enum BBCollisionPoint {
    Top, 
    Bottom, 
    Left, 
    Right, 
}

pub struct BrickColumn {
    bricks: Vec<BrickRow>, 
}
impl BrickColumn {
    pub fn spawn(
        column: u32, 
        row: u32, 
        margin_top: f32, 
        brick_margin: nalgebra::Vector2<f32>, 
        brick_size: nalgebra::Vector2<f32>, 
        disp_size: nalgebra::Vector2<f32>, 
        spawn_f: Arc<PMutex<
            impl FnMut([u32; 2], nalgebra::Point2<f32>, nalgebra::Vector2<f32>) -> Option<Brick>
        >>, 
    ) -> Self {
        let height = column as f32 * brick_size.y + (column - 1) as f32 * brick_margin.y;
        let y_pos = disp_size.y - (margin_top + height);
        let bricks = (0..column).into_iter()
            .map(|y| {
                (y, brick_margin.y * y as f32, brick_size.y * y as f32)
            })
            .map(|(column, padding_bottom, pos_bottom)|
                BrickRow::spawn(
                    row, 
                    column, 
                    padding_bottom + pos_bottom + y_pos + brick_size.y / 2., 
                    brick_margin.x, 
                    brick_size, 
                    disp_size, 
                    Arc::clone(&spawn_f)
                )
            ).collect();

        Self { bricks }
    }
    pub fn collision(&mut self, ball: &super::ball::Ball) -> Option<BBCollisionPoint> {
        for b in self.bricks.iter_mut()
            .map(|b| b.collision(ball))
        {
            if let Some(r) = b { return Some(r) }
        }
        None
    }
    pub fn count(&self) -> usize {
        let mut count = 0;
        self.bricks.iter()
            .map(|b| b.count)
            .for_each(|c| count += c);
        count
    }
}
impl super::AsInstance for BrickColumn {
    fn as_instance(&self, instances: &mut crate::gfx::model::RawInstArray) {
        self.bricks.iter().for_each(|b| b.as_instance(instances))
    }
}

/// ブロックの列
pub struct BrickRow {
    bricks: Vec<Option<Brick>>, 
    count: usize, 
}
impl BrickRow {
    pub fn spawn(
        row: u32, 
        column: u32, 
        pos_y: f32, 
        brick_margin: f32, 
        brick_size: nalgebra::Vector2<f32>, 
        disp_size: nalgebra::Vector2<f32>, 
        spawn_f: Arc<PMutex<
            impl FnMut([u32; 2], nalgebra::Point2<f32>, nalgebra::Vector2<f32>) -> Option<Brick>
        >>, 
    ) -> Self {
        let width = row as f32 * brick_size.x + (row - 1) as f32 * brick_margin;
        let margin_left = (disp_size.x - width) / 2.;
        let mut count = 0;
        let bricks = (0..row).into_iter()
            .map(|x| (x, brick_margin * x as f32, brick_size.x * x as f32))
            .map(|(row, padding_left, pos_left)| (spawn_f.lock())(
                [row, column], 
                [padding_left + margin_left + pos_left + brick_size.x / 2., pos_y].into(), 
                brick_size, 
            ))
            .map(|e| {
                if let Some(_) = e { count += 1; };
                e
            })
            .collect();

        Self { 
            bricks, 
            count, 
        }
    }
    pub fn collision(&mut self, ball: &super::ball::Ball) -> Option<BBCollisionPoint> {
        for b in self.bricks.iter_mut()
        {
            if let Some(rb) = if let Some(rb) = b {
                rb.collision(ball)
            } else {
                None
            } {
                self.count -= 1;
                *b = None;
                return Some(rb)
            }
        }
        None
    }
}
impl super::AsInstance for BrickRow {
    fn as_instance(&self, instances: &mut crate::gfx::model::RawInstArray) {
        self.bricks.iter()
            .filter_map(|b| b.as_ref())
            .for_each(|b| b.as_instance(instances))
    }
}

/// ブロック
pub struct Brick {
    model: super::Instance, 
}
impl Brick {
    pub fn spawn(
        position: nalgebra::Point2<f32>, 
        size: nalgebra::Vector2<f32>, 
        color: [f32; 4], 
    ) -> Self { Self { 
        model: super::Instance {
            position,
            size,
            angle: 0.,
            color,
        }
    } }
    pub fn collision(
        &self, 
        ball: &super::ball::Ball
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
}
impl super::AsInstance for Brick {
    fn as_instance(&self, instances: &mut crate::gfx::model::RawInstArray) {
        instances.push(&self.model)
    }
}

pub(super) fn brick_spawner(
    pos: [u32; 2], 
    blkpos: nalgebra::Point2<f32>, 
    blksize: nalgebra::Vector2<f32>, 
) -> Option<Brick> {
    if pos[1] % 3 == 0 {
        None
    } else {
        Some(Brick::spawn(
            blkpos, 
            blksize, 
            [
                1. - pos[1] as f32 * (1. / 18.), 
                pos[0] as f32 * (1. / 18.), 
                pos[1] as f32 * (1. / 18.), 
                1.
            ]
        ))
    }
}