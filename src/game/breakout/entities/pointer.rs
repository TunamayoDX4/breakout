/// ボールの着弾地点の表示
pub struct Pointer {
    pub(super) model: super::Instance, 
    pub(super) visible: bool, 
}
impl Pointer {
    pub fn spawn() -> Self { Self {
        model: super::Instance {
            position: [0., 0.].into(),
            size: [8., 8.].into(),
            angle: 0.,
            color: [1., 0., 0., 1.],
        },
        visible: false, 
    }}
}
impl super::AsInstance for Pointer {
    fn as_instance(&self, instances: &mut super::RawInstArray) {
        if self.visible { instances.push(&self.model) }
    }
}