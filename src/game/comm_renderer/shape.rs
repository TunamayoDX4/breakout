#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Dot {
    vert: DotVertex, 
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Line {
    vert: [DotVertex; 2], 
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Polygon {
    vert: [PolygonVertex; 3], 
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Rectangle {
    pos: [f32; 4], 
    size: [f32; 2], 
    rotation: [f32; 2], 
    vert: [RectangleVertex; 4], 
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Quadangle {
    vert: [PolygonVertex; 4], 
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct DotVertex {
    pos: [f32; 4], 
    color: [f32; 4], 
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct PolygonVertex {
    pos: [f32; 4], 
    color: [f32; 4], 
    uv: [f32; 2], 
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectangleVertex {
    color: [f32; 4], 
    uv: [f32; 2], 
}