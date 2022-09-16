//! ゲームの状態

pub struct GameState {
    pub ipaexg: super::util::text_renderer::TextRendererGMArc,  
    pub ipaexg_: super::util::text_renderer::TextRenderer, 
}
impl GameState {
    pub(super) fn new(
        gfx_ctx: &crate::gfx::WGContext, 
    ) -> anyhow::Result<Self> { 
        let ipaexg = super::util::text_renderer::TextRendererGMArc::new(
            gfx_ctx, 
            include_bytes!("../../../ipaexg.ttf").to_vec(), 
        )?;
        let ipaexg_ = super::util::text_renderer::TextRenderer::new(
            gfx_ctx, 
            Some({
                let mut map = hashbrown::HashMap::new();
                map.insert(
                    "sample".into(), 
                    super::util::text_renderer::TextEntry::new(
                        [32., 32.], 
                        vec![
                            super::util::text_renderer::TextObj {
                                text: "Hello, wgpu glyph!".into(), 
                                color: [1., 1., 1., 1.], 
                                scale: 16., 
                            }, 
                            super::util::text_renderer::TextObj {
                                text: "Hello, wgpu glyph!".into(), 
                                color: [1., 1., 1., 1.], 
                                scale: 32., 
                            }, 
                        ], 
                        wgpu_glyph::Layout::default()
                    )
                );
                map.insert(
                    "sample2".into(), 
                    super::util::text_renderer::TextEntry::new(
                        [32., 128.], 
                        vec![
                            super::util::text_renderer::TextObj {
                                text: "Hello, wgpu glyph!".into(), 
                                color: [1., 1., 1., 1.], 
                                scale: 16., 
                            }, 
                            super::util::text_renderer::TextObj {
                                text: "Hello, wgpu glyph!".into(), 
                                color: [1., 1., 1., 1.], 
                                scale: 32., 
                            }, 
                        ], 
                        wgpu_glyph::Layout::default()
                    )
                );
                map
            }), 
            ipaexg.clone(),  
        )?;
        Ok(Self {
            ipaexg, 
            ipaexg_, 
        })
    }
}