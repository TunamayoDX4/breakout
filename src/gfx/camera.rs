//! カメラモジュール

/// カメラ構造体
pub struct Camera {
    pub pos: nalgebra::Point2<f32>, 
    pub view_port: nalgebra::Vector2<f32>,  
    pub angle: f32, 
    pub scale: f32, 
}
impl Camera {
    pub fn update(&mut self, view_size: &winit::dpi::PhysicalSize<u32>) {
        self.view_port.x = view_size.width as f32;
        self.view_port.y = view_size.height as f32;
    }
    fn as_raw(&self) -> CameraMat {
        let view_port = &self.view_port;
        let view_scale = nalgebra::Vector2::new(
            1. / view_port.x, 
            1. / view_port.y, 
        );
        let pos = &self.pos;
        let angle = &self.angle;
        let scale = view_scale * self.scale;
        let ret = CameraMat([
            [scale.x * angle.cos(), scale.y * -angle.sin(), scale.x * pos.x, 0.], 
            [scale.x * angle.sin(), scale.y * angle.cos(), scale.y * pos.y, 0.], 
            [0., 0., 1., 0.],  
            [view_port.x * 0.5, view_port.y * 0.5, 0., 0.], 
        ]);
        ret
    }
}

/// カメラの生配列
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraMat ([[f32; 4]; 4]);
impl CameraMat {
    pub fn new() -> Self { Self(Default::default()) }
    pub fn update(&mut self, camera: &Camera) {
        *self = camera.as_raw();
    }
}