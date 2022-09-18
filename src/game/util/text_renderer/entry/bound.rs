//! テキストの領域サイズ

#[derive(Default)]
pub enum TextBound {
    #[default]
    DispSize, 
    CustomSize(nalgebra::Vector2<f32>), 
}

impl TextBound {
    pub fn to_size(&self, disp_size: nalgebra::Vector2<f32>) -> (f32, f32) {
        let bound_size = match self {
            TextBound::DispSize => disp_size,
            TextBound::CustomSize(size) => *size,
        };
        (bound_size.x, bound_size.y)
    }
}