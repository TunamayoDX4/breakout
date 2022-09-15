//! グラフィクスモジュール

use wgpu::{Surface, Device, Queue, SurfaceConfiguration, Instance, Backends, RenderPipeline, Buffer, util::DeviceExt, BindGroup, PowerPreference, Features, Limits, DeviceDescriptor, PresentMode, Adapter};
use winit::{dpi::PhysicalSize, window::Window};

/// 描画処理の実装
pub mod rendering;

/// モデルの実装
pub mod model;

/// カメラの実装
pub mod camera;

pub trait WGRenderer {
    fn rendering(
        &mut self, 
        output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
        queue: &Queue, 
    );
}

pub struct WGContextNew {
    pub surface: Surface, 
    pub device: Device, 
    pub adapter: Adapter, 
    pub queue: Queue, 
    pub config: SurfaceConfiguration, 
    pub size: PhysicalSize<u32>, 
}
impl WGContextNew {
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
    pub(super) fn rendering<R, T>(
        &self, 
        renderer: R, 
        extractor: impl Fn(&mut T) -> &mut dyn WGRenderer
    ) -> Result<(), wgpu::SurfaceError> where 
        R: IntoIterator<Item = T>, 
    {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        renderer.into_iter().for_each(|mut r| extractor(&mut r).rendering(
            &output, 
            &view, 
            &self.queue
        ));
        output.present();
        Ok(())
    }
}

/// WGPUのコンテキスト
pub struct WGContext {
    surface: Surface, 
    device: Device, 
    queue: Queue, 
    config: SurfaceConfiguration, 
    size: PhysicalSize<u32>, 
    render_pipeline: RenderPipeline, 
    vertex_buffer: Buffer, 
    index_buffer: Buffer, 
    num_indices: u32, 
    raw_instances: model::RawInstArray, 
    instances_buffer: Buffer, 
    camera: camera::Camera, 
    camera_mat: camera::CameraMat, 
    camera_buffer: Buffer, 
    camera_bg: BindGroup, 
    ipaexg_glyph: wgpu_glyph::GlyphBrush<()>, 
    font_belt: wgpu::util::StagingBelt, 
}
impl WGContext {
    /// コンテキストの初期化
    pub(super) async fn new(window: &Window) -> anyhow::Result<Self> {
    
        /***
            コンテキストの初期化
        ***/

        let size = window.inner_size();

        // インスタンス
        let instance = Instance::new(Backends::all());

        // サーフェス
        let surface = unsafe { instance.create_surface(window) };

        // アダプタ
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
        })
            .await
            .ok_or(anyhow::anyhow!("WGPU adapter request failure"))?;

        // デバイスおよびキュー
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            }, 
            None
        )   
            .await?;
        
        // コンフィグ
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        // コンフィグの適用
        surface.configure(&device, &config);

        /***
            バッファ類の初期化
        ***/

        // 頂点バッファの初期化
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(model::VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // インデックスバッファの初期化
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index buffer"),
                contents: bytemuck::cast_slice(model::INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = model::INDICES.len() as u32;

        // インスタンス配列の初期化
        let raw_instances = model::RawInstArray::new();

        // インスタンスバッファの初期化
        let instances_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance buffer"),
                contents: bytemuck::cast_slice(raw_instances.get()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // カメラの初期化
        let camera = camera::Camera {
            pos: nalgebra::Point2::new(-0., 0.),
            view_port: nalgebra::Vector2::new(size.width as f32, size.height as f32),
            angle: 0.,
            scale: 1.,
        };
        // 行列の初期化
        let mut camera_mat = camera::CameraMat::new();
        camera_mat.update(&camera);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera buffer"),
                contents: bytemuck::cast_slice(&[camera_mat]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bg_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera bindgroup Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
            }
        );
        let camera_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera bindgroup"),
            layout: &&camera_bg_layout, 
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
        });

        // フォント
        let ipaexg = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../../ipaexg.ttf"
        ))?;

        let ipaexg_glyph = wgpu_glyph::GlyphBrushBuilder::using_font(ipaexg)
            .build(&device, surface.get_supported_formats(&adapter)[0]);
        
        let font_belt = wgpu::util::StagingBelt::new(1024);

        /***
            パイプライン・レンダラーの初期化
        ***/

        // シェーダの初期化
        let shader = device.create_shader_module(
            wgpu::include_wgsl!("../../shader/main.wgsl")
        );

        // 描画パイプラインのレイアウト
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[
                    &camera_bg_layout, 
                ],
                push_constant_ranges: &[],
        });

        // 描画パイプライン
        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        model::Vertex::desc(), 
                        model::InstanceRaw::desc(), 
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::all(),
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline, 
            vertex_buffer,
            index_buffer,
            num_indices, 
            raw_instances, 
            instances_buffer,
            camera,
            camera_mat,
            camera_buffer,
            camera_bg,
            ipaexg_glyph,
            font_belt, 
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
}