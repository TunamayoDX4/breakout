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
							[16., 16.], 
							vec![
								TextObj::new(
									" BreakOut ブロック崩し ", 
									16., 
									[1., 1., 1., 1.], 
								), 
								TextObj::new(
									"", 
									16., 
									[1., 1., 1., 1.], 
								), 
								TextObj::new(
									"", 
									16., 
									[1., 1., 1., 1.], 
								), 
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
									" Escキーで終了 ", 
									16., 
									[1., 1., 1., 1.], 
								), 
								TextObj::new(
									" ADキー/カーソル移動で移動 ", 
									16., 
									[1., 1., 1., 1.], 
								), 
								TextObj::new(
									" Spaceキー/左クリックで玉発射 ", 
									16., 
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