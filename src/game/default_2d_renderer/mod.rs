use super::comm_renderer;

pub mod shape;

pub enum D2DRendererComm {
	Dot(shape::Dot), 
	Line(shape::Line), 
	Triangle(shape::Triangle), 
}
impl comm_renderer::RenderCommand for D2DRendererComm {
    type Ctx = D2DRendererCtx;

    fn rendering(
        self, 
        ctx: &Self::Ctx, 
        wgpu_ctx: &crate::gfx::WGContext, 
        output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
    ) { match self {
        D2DRendererComm::Dot(dot) => dot.rendering(wgpu_ctx, view, &ctx.dot),
        D2DRendererComm::Line(line) => line.rendering(wgpu_ctx, view, &ctx.line),
        D2DRendererComm::Triangle(tri) => tri.rendering(wgpu_ctx, view, &ctx.triangle),
    }}
}

pub struct D2DRendererCtx {
	dot: shape::DotRenderer, 
	line: shape::LineRenderer, 
	triangle: shape::TriangleRenderer, 
}
impl comm_renderer::RenderCtx for D2DRendererCtx {
    type Comm = D2DRendererComm;
}