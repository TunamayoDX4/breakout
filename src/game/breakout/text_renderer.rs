//! テキストのレンダラ

use super::super::util::text_renderer::{
	TextRenderer, 
	TextRendererGMArc, 
	entry::{
		TextEntry, 
		TextObj, 
		body::TextBody, 
	}, 
};

pub struct BreakOutGameTextRenderer {
	renderer: TextRenderer, 
}
impl BreakOutGameTextRenderer {
	pub fn new(
		gfx_ctx: &crate::gfx::WGContext, 
		glyph: TextRendererGMArc, 
	) -> anyhow::Result<Self> { 
		Ok(Self {
			renderer: TextRenderer::new(
				gfx_ctx, 
				Some({
					let mut entries = hashbrown::HashMap::new();
					entries.insert(
						"".into(),
						TextEntry::new(
							[32., 32.], 
							vec![
								TextObj {
									text: TextBody::from_direct("".into()), 
									color: [1., 1., 1., 1.], 
									scale: 32., 
								}
							], 
							wgpu_glyph::Layout::default()
						)
					);
					entries
				}), 
				glyph, 
			)?
		})
	}
	pub fn entry(&self, key: &str) -> Option<&TextEntry> {
		self.renderer.get_entry().get(key)
	}
	pub fn entry_mut(&mut self, key: &str) -> Option<&mut TextEntry> {
		self.renderer.get_entry_mut().get_mut(key)
	}
}
impl crate::gfx::WGRenderer for BreakOutGameTextRenderer {
    fn rendering(
        &mut self, 
        output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
        ctx: &crate::gfx::WGContext, 
    ) {
        self.renderer.rendering(output, view, ctx)
    }
}