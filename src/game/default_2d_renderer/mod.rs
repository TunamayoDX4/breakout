use super::comm_renderer;

pub mod shape;

pub enum D2DRendererComm {
	Dot(shape::dot::Dot), 
	Line(shape::line::Line), 
	Triangle(shape::triangle::Triangle), 
}
impl comm_renderer::RenderCommand for D2DRendererComm {
    type Ctx = D2DRendererCtx;

    fn rendering(
        self, 
        ctx: &Self::Ctx, 
        wgpu_ctx: &crate::gfx::WGContext, 
        _output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
    ) { match self {
        D2DRendererComm::Dot(dot) => dot.rendering(
            wgpu_ctx, 
            view, 
            &ctx.dot
        ),
        D2DRendererComm::Line(line) => line.rendering(
            wgpu_ctx, 
            view, 
            &ctx.line
        ),
        D2DRendererComm::Triangle(tri) => tri.rendering(
            wgpu_ctx, 
            view, 
            &ctx.triangle
        ),
    }}
}

pub struct D2DRendererCtx {
	dot: shape::dot::DotRenderer, 
	line: shape::line::LineRenderer, 
	triangle: shape::triangle::TriangleRenderer, 
}
impl comm_renderer::RenderCtx for D2DRendererCtx {
    type Comm = D2DRendererComm;
}
impl D2DRendererCtx {
    pub fn new(wgpu_ctx: &crate::gfx::WGContext) -> Self {
        Self {
            dot: shape::dot::DotRenderer::new(wgpu_ctx), 
            line: shape::line::LineRenderer::new(wgpu_ctx), 
            triangle: shape::triangle::TriangleRenderer::new(wgpu_ctx), 
        }
    }
}