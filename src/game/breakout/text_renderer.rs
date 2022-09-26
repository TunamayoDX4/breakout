//! テキストのレンダラ

use super::super::util::text_renderer::{
	TextRenderer, 
	TextRendererGMArc, 
	entry::{
		bound::TextBound, 
		TextEntry, 
		TextObj, 
	}, 
};

pub struct BreakOutGameTextRenderer {
	renderer: TextRenderer, 
}
impl BreakOutGameTextRenderer {
	pub fn new(
		glyph: TextRendererGMArc, 
	) -> anyhow::Result<Self> { 
		Ok(Self {
			renderer: TextRenderer::new(
				Some({
					let mut entries = hashbrown::HashMap::new();
					entries.insert(
						"top".into(),
						TextEntry::new(
							TextBound::DispSize, 
							[16., 8.], 
							vec![
								TextObj::new(
									" BreakOut ", 
									20., 
									[1., 1., 1., 1.], 
								), 
								TextObj::new(
									"", 
									20., 
									[1., 1., 1., 1.], 
								), 
								TextObj::new(
									"", 
									20., 
									[1., 1., 1., 1.], 
								), 
								TextObj::new(
									"", 
									20., 
									[1., 1., 1., 1.], 
								)
							], 
							wgpu_glyph::Layout::default()
						)
					);
					entries.insert(
						"bottom".into(), 
						TextEntry::new(
							TextBound::DispSize, 
							[16., 608.], 
							vec![
								TextObj::new(
									" Esc to Exit ", 
									20., 
									[1., 1., 1., 1.], 
								), 
								TextObj::new(
									" AD or Cursor: Move ", 
									20., 
									[1., 1., 1., 1.], 
								), 
								TextObj::new(
									" Space or Left-click : Shoot ", 
									20., 
									[1., 1., 1., 1.], 
								), 
							], 
							wgpu_glyph::Layout::default(), 
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