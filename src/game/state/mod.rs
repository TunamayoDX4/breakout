//! ゲームの状態

use super::util::text_renderer::{
    TextRenderer, 
    TextRendererGMArc, 
};

pub struct GameState {
    pub ipaexg: TextRendererGMArc,  
    pub ipaexg_: TextRenderer, 
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
                        ], 
                        wgpu_glyph::Layout::default()
                    )
                );
                map.insert(
                    "sample2".into(), 
                    super::util::text_renderer::TextEntry::new(
                        [32., 128.], 
                        vec![
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