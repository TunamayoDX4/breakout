//! サウンドエフェクトモジュール

use std::{sync::Arc, borrow::Cow};

use parking_lot::RwLock;
use rodio::{
    dynamic_mixer::{
        mixer, 
        DynamicMixerController, 
    }, 
    OutputStream, 
    Source, 
    OutputStreamHandle, 
    Sink, 
    source::{Zero, Buffered}, 
    Decoder, 
};
use hashbrown::HashMap;

/// サウンドエフェクトモジュール
#[derive(Clone)]
pub struct SfxModule(Arc<RwLock<SfxModuleInner>>);
impl SfxModule {
    pub fn new(volume: f32) -> anyhow::Result<Self> {
        Ok(Self(Arc::new(RwLock::new(
            SfxModuleInner::new(volume)?
        ))))
    }
    pub fn re_configure(&mut self, volume: f32) {
        self.0.write().re_configure(volume)
    }
    pub fn play<T: Source<Item = f32> + Send + 'static>(&self, src: T) {
        self.0.read().play(src)
    }
    pub fn add_resource<T: Into<Cow<'static, str>>> (
        &mut self, 
        name: T, 
        file: Decoder<std::fs::File>
    ) -> Option<Buffered<Decoder<std::fs::File>>> {
        self.0.write().add_resource(name, file)
    }
    pub fn play_resource<T: Source<Item = S> + Send + 'static, S: rodio::Sample> (
        &self, 
        name: &str, 
        f: impl FnMut(Buffered<Decoder<std::fs::File>>) -> T, 
    ) -> bool {
        self.0.read().play_resource(name, f)
    }
}

/// サウンドエフェクトモジュールの内部型
struct SfxModuleInner {
    _stream: OutputStream, 
    _stream_handle: OutputStreamHandle, 
    sink: Sink, 
    mixer_ctrl: Arc<DynamicMixerController<f32>>, 
    res_mngr: SfxResourceMngrInner, 
}
impl SfxModuleInner {
    pub fn new(volume: f32) -> anyhow::Result<Self> {
        let (
            stream, 
            stream_handle
        ) = OutputStream::try_default()?;
        let (
            mixer_ctrl, 
            mixer, 
        ) = mixer(16, 44_100);
        let sink = Sink::try_new(&stream_handle)?;
        // バックで常に音量0で流し続ける
        mixer_ctrl.add(Zero::new(16, 44_100));
        sink.set_volume(volume);
        sink.append(mixer);
        sink.play();
        let res_mngr = SfxResourceMngrInner::new();
        Ok(Self {
            _stream: stream,
            _stream_handle: stream_handle,
            sink, 
            mixer_ctrl,
            res_mngr, 
        })
    }
    pub fn re_configure(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }
    pub fn play<T: Source<Item = f32> + Send + 'static>(&self, src: T) {
        self.mixer_ctrl.add(src)
    }
    pub fn add_resource<T: Into<Cow<'static, str>>> (
        &mut self, 
        name: T, 
        file: Decoder<std::fs::File>
    ) -> Option<Buffered<Decoder<std::fs::File>>> {
        self.res_mngr.add(name, file)
    }
    pub fn play_resource<T: Source<Item = S> + Send + 'static, S: rodio::Sample> (
        &self, 
        name: &str, 
        f: impl FnMut(Buffered<Decoder<std::fs::File>>) -> T, 
    ) -> bool {
        self.res_mngr.play(name, self, f)
    }
}

/// サウンドエフェクトの音声データのマネージャの内部型
struct SfxResourceMngrInner {
    resources: HashMap<
        Cow<'static, str>, 
        Buffered<Decoder<std::fs::File>>, 
    >
}
impl SfxResourceMngrInner {
    fn new() -> Self { Self { resources: Default::default() } }
    fn add<T: Into<Cow<'static, str>>>(
        &mut self, 
        name: T, 
        file: Decoder<std::fs::File>
    ) -> Option<Buffered<Decoder<std::fs::File>>>{
        self.resources.insert(
            name.into(), 
            file.buffered()
        )
    }
    fn play<T: Source<Item = S> + Send + 'static, S: rodio::Sample>(
        &self, 
        name: &str, 
        sfx_ctx: &SfxModuleInner, 
        mut f: impl FnMut(Buffered<Decoder<std::fs::File>>) -> T
    ) -> bool {
        self.resources.get(name)
            .map(|s| sfx_ctx.play(
                f(s.clone()).convert_samples()
            ))
            .is_some()
    }
}