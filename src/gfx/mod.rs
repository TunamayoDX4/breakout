//! グラフィクスモジュール

use wgpu::{Surface, Device, Queue, SurfaceConfiguration, Instance, Backends, PowerPreference, Features, Limits, DeviceDescriptor, PresentMode, Adapter};
use winit::{dpi::PhysicalSize, window::Window};

pub trait WGRenderer {
    fn rendering(
        &mut self, 
        output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
        ctx: &WGContext, 
    );
}

pub struct WGContext {
    pub surface: Surface, 
    pub device: Device, 
    pub adapter: Adapter, 
    pub queue: Queue, 
    pub config: SurfaceConfiguration, 
    pub size: PhysicalSize<u32>, 
}
impl WGContext {
    /// コンテキストの初期化
    pub(super) async fn new(window: &Window) -> anyhow::Result<Self> {
        // コンテキストの初期化
        let size = window.inner_size();
        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }
        ).await
            .ok_or("WGPU adapter request failure")
            .map_err(|e| anyhow::anyhow!(e))?;
        let (device, queue) = adapter.request_device(
            &DeviceDescriptor { 
                label: None, 
                features: Features::empty(), 
                limits: Limits::default() 
            }, 
            None
        ).await?;
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
        };
        Ok(Self {
            surface,
            device,
            adapter,
            queue,
            config,
            size,
        })
    }

    /// 描画領域のリサイズ
    pub(super) fn resize(
        &mut self, 
        new_size: PhysicalSize<u32>
    ) { if new_size.width > 0 && new_size.height > 0 {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }}
    
    /// コンフィグの再読取り
    pub(super) fn re_configure(&mut self) {
        self.surface.configure(&self.device, &self.config);
    }

    /// 画面サイズの取得
    pub(super) fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    /// 描画処理
    pub(super) fn rendering(
        &self, 
    ) -> Result<RenderingChain, wgpu::SurfaceError> 
    {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        Ok(RenderingChain { output, view })
    }
}

/// 描画処理のチェーン
pub struct RenderingChain {
    output: wgpu::SurfaceTexture, 
    view: wgpu::TextureView, 
}
impl RenderingChain {
    pub fn rendering<'b, R>(self, ctx: &WGContext, renderer: R) -> Self where
        R: IntoIterator<Item = &'b mut dyn WGRenderer>
    {
        renderer.into_iter()
            .for_each(|r| r.rendering(
                &self.output, 
                &self.view, 
                ctx
            ));
        Self { output: self.output, view: self.view }
    }
    pub fn present(self) {
        self.output.present()
    }
}