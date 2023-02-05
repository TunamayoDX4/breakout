//! モデルの実装

/// モデルの頂点データ
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 4], 
    pub color: [f32; 4], 
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x4, 1 => Float32x4
    ];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &Self::ATTRIBS, 
        }
    }
}

/// 頂点配列
pub const VERTICES: &[Vertex] = &[
    Vertex{ pos: [1.0, 1.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] }, 
    Vertex{ pos: [-1.0, 1.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] }, 
    Vertex{ pos: [-1.0, -1.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] }, 
    Vertex{ pos: [1.0, -1.0, 0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] }, 
];

/// インデックス配列
pub const INDICES: &[u16] = &[
    0, 1, 2, 
    2, 3, 0, 
];

/// 生インスタンスの配列型
pub struct RawInstArray(Vec<InstanceRaw>);
impl RawInstArray {
    pub(super) fn new() -> Self { Self(Vec::new()) }
    pub(super) fn init(&mut self) { self.0.clear() }
    pub(super) fn get(&self) -> &[InstanceRaw] { self.0.as_slice() }
    pub(super) fn len(&self) -> usize { self.0.len() }
    pub fn push(&mut self, instance: &Instance) {
        self.0.push(instance.to_raw())
    }
}

/// インスタンスの参照を得られる型
pub trait AsInstance {
    fn as_instance(&self, instances: &mut RawInstArray);
}

/// インスタンス
pub struct Instance {
    pub position: nalgebra::Point2<f32>, 
    pub size: nalgebra::Vector2<f32>, 
    pub angle: f32, 
    pub color: [f32; 4], 
}
impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            position: [
                self.position.x, 
                self.position.y
            ],
            size: [self.size.x / 2., self.size.y / 2.], 
            rotation: [self.angle.cos(), self.angle.sin()], 
            color: self.color.clone(),
        }
    }
    pub fn collision_aabb(&self, other: &Self) -> bool {
        self.position.x - self.size.x / 2. <= other.position.x + other.size.x / 2.
        && self.position.y - self.size.y / 2. <= other.position.y + other.size.y / 2.
        && other.position.x - other.size.x / 2. <= self.position.x + self.size.x / 2.
        && other.position.y - other.size.y / 2. <= self.position.y + self.size.y / 2.
    }
    pub fn edges(&self) -> [[nalgebra::Point2<f32>; 2]; 4] { 
        let self_size = self.size * 0.5;
        [[
            [self.position.x - self_size.x, self.position.y - self_size.y].into(), 
            [self.position.x + self_size.x, self.position.y - self_size.y].into(), 
        ], [
            [self.position.x - self_size.x, self.position.y + self_size.y].into(), 
            [self.position.x + self_size.x, self.position.y + self_size.y].into(), 
        ], [
            [self.position.x - self_size.x, self.position.y - self_size.y].into(), 
            [self.position.x - self_size.x, self.position.y + self_size.y].into(), 
        ], [
            [self.position.x + self_size.x, self.position.y - self_size.y].into(), 
            [self.position.x + self_size.x, self.position.y + self_size.y].into(),
        ]]
    }
}
impl AsRef<Instance> for Instance {
    fn as_ref(&self) -> &Instance { self }
}

/// 生インスタンス
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    position: [f32; 2], 
    size: [f32; 2], 
    rotation: [f32; 2], 
    color: [f32; 4], 
}
impl InstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        5 => Float32x2, 
        6 => Float32x2, 
        7 => Float32x2, 
        8 => Float32x4, 
    ];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}