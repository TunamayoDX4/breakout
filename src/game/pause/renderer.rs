//! ポーズ画面のレンダラ

use super::super::util::text_renderer::{
    TextRenderer, 
    TextRendererGMArc, 
    entry::{
        bound::TextBound, 
        TextEntry, 
        TextObj, 
    }, 
};

pub struct PauseRenderer {
    renderer: TextRenderer, 
}
impl PauseRenderer {
    pub fn new(
        glyph: TextRendererGMArc, 
    ) -> anyhow::Result<Self> {
        Ok(Self {
            renderer: TextRenderer::new(
                Some({
                    let mut entries = hashbrown::HashMap::new();
                    entries.insert(
                        "center".into(), 
                        TextEntry::new(
                            TextBound::DispSize, 
                            [320., 320.], 
                            vec![
                                TextObj::new(
                                    "PAUSE\n\n", 
                                    16., 
                                    [1., 1., 1., 1.]
                                ), 
                                TextObj::new(
                                    "Oキーで戻る", 
                                    16., 
                                    [1., 1., 1., 1.]
                                ), 
                            ], 
                            wgpu_glyph::Layout::Wrap { 
                                line_breaker: wgpu_glyph::BuiltInLineBreaker::default(), 
                                h_align: wgpu_glyph::HorizontalAlign::Center, 
                                v_align: wgpu_glyph::VerticalAlign::Center 
                            }
                        )
                    );
                    entries
                }), 
                glyph, 
            )?
        })
    }
}
impl crate::gfx::WGRenderer for PauseRenderer {
    fn rendering(
        &mut self, 
        output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
        ctx: &crate::gfx::WGContext, 
    ) {
        self.renderer.rendering(output, view, ctx)
    }
}