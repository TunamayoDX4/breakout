type PMutex<T> = parking_lot::Mutex<T>;
use std::sync::Arc;

use super::{Instance, AsInstance};

/// ブロックそのものの実装
pub mod brick;
pub use brick::Brick;

/// ブロックとボールのだいたいの接触位置
pub enum BBCollisionPoint {
    Top, 
    Bottom, 
    Left, 
    Right, 
}

/// ブロック配列のラップ型
pub struct BrickCollection(BrickColumn);
impl BrickCollection {
    pub fn spawn<BM, BS, SF>(
        disp_size: nalgebra::Vector2<f32>, 
        param: BrickSpawnParam<BM, BS, SF>, 
    ) -> Self where
        BM: Into<nalgebra::Vector2<f32>>, 
        BS: Into<nalgebra::Vector2<f32>>, 
        SF: FnMut(
            [u32; 2], 
            nalgebra::Point2<f32>, 
            nalgebra::Vector2<f32>
        ) -> Option<Brick>, 
    {
        Self(BrickColumn::spawn(
            param.column, 
            param.row, 
            param.margin_top, 
            param.brick_margin.into(), 
            param.brick_size.into(), 
            disp_size, 
            param.spawn_f, 
        ))
    }
    pub fn get(&self) -> &BrickColumn { &self.0 }
    pub fn get_mut(&mut self) -> &mut BrickColumn { &mut self.0 }
}
impl super::AsInstance for BrickCollection {
    fn as_instance(&self, instances: &mut super::RawInstArray) {
        self.0.as_instance(instances)
    }
}

/// ブロックの行
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
    pub fn collision(
        &mut self, 
        ball: &super::ball::Ball, 
        f: impl FnMut(&Brick) + Clone, 
    ) -> Option<BBCollisionPoint> {
        for b in self.bricks.iter_mut()
            .map(|b| b.collision(ball, f.clone()))
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
    fn as_instance(&self, instances: &mut super::RawInstArray) {
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
    pub fn collision(
        &mut self, 
        ball: &super::ball::Ball, 
        mut f: impl FnMut(&Brick), 
    ) -> Option<BBCollisionPoint> {
        for b in self.bricks.iter_mut()
        {
            if let Some(rb) = if let Some(rb) = b {
                rb.collision(ball)
            } else {
                None
            } {
                if let Some(b) = b { f(b) }
                self.count -= 1;
                *b = None;
                return Some(rb)
            }
        }
        None
    }
}
impl super::AsInstance for BrickRow {
    fn as_instance(&self, instances: &mut super::RawInstArray) {
        self.bricks.iter()
            .filter_map(|b| b.as_ref())
            .for_each(|b| b.as_instance(instances))
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
            ], 
            100 * pos[1] as u64, 
        ))
    }
}

/// ブロックのスポーン時に引き渡す値
pub struct BrickSpawnParam<
    BM: Into<nalgebra::Vector2<f32>>, 
    BS: Into<nalgebra::Vector2<f32>>, 
    SF: FnMut(
        [u32; 2], 
        nalgebra::Point2<f32>, 
        nalgebra::Vector2<f32>
    ) -> Option<Brick>, 
> {
    pub column: u32, 
    pub row: u32, 
    pub margin_top: f32, 
    pub brick_margin: BM, 
    pub brick_size: BS, 
    pub spawn_f: Arc<parking_lot::Mutex<SF>>, 
}