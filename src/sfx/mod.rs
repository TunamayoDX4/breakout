//! サウンドエフェクトモジュール

use std::sync::Arc;

use parking_lot::RwLock;
use rodio::{
    dynamic_mixer::{
        mixer, 
        DynamicMixerController, 
    }, 
    OutputStream, 
    Source, 
    OutputStreamHandle, Sink, source::Zero
};

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
    pub fn add<T: Source<Item = f32> + Send + 'static>(&self, src: T) {
        self.0.read().add(src)
    }
}

/// サウンドエフェクトモジュールの内部型
struct SfxModuleInner {
    _stream: OutputStream, 
    _stream_handle: OutputStreamHandle, 
    sink: Sink, 
    mixer_ctrl: Arc<DynamicMixerController<f32>>, 
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
        sink.append(mixer.repeat_infinite());
        sink.play();
        Ok(Self {
            _stream: stream,
            _stream_handle: stream_handle,
            sink, 
            mixer_ctrl,
        })
    }
    pub fn re_configure(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }
    pub fn add<T: Source<Item = f32> + Send + 'static>(&self, src: T) {
        self.mixer_ctrl.add(src)
    }
}