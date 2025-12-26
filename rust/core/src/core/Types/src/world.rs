use crate::core::types::base_camera::BaseCamera;
use crate::core::types::base_renderer::BaseRenderer;
use crate::core::types::base_scene::{BaseScene, Mesh};
use crate::core::types::event::Event;
use crate::core::types::interfaces::{Disposable, Updateable};
use std::sync::{Arc, Mutex};

pub type WorldHandle = Arc<Mutex<World>>;

pub struct World {
    pub meshes: Vec<Mesh>,
    pub scene: Arc<Mutex<BaseScene>>,
    pub default_camera: Arc<Mutex<BaseCamera>>,
    pub camera: Arc<Mutex<BaseCamera>>,
    pub on_camera_changed: Event<()>,
    pub renderer: Option<Arc<Mutex<BaseRenderer>>>,
    pub uuid: String,
    pub is_disposing: bool,
    pub on_disposed: Event<()>,
    pub on_after_update: Event<()>,
    pub on_before_update: Event<()>,
}

impl World {
    pub fn use_default_camera(&mut self) {
        self.camera = Arc::clone(&self.default_camera);
        self.on_camera_changed.trigger((), None);
    }
}

impl Disposable for World {
    fn dispose(&mut self) {
        self.is_disposing = true;
        self.on_disposed.trigger((), None);
        self.on_disposed.reset();
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}

impl Updateable for World {
    fn on_after_update(&self) -> &Event<()> {
        &self.on_after_update
    }

    fn on_before_update(&self) -> &Event<()> {
        &self.on_before_update
    }

    fn update(&mut self, delta: Option<f32>) {
        self.on_before_update.trigger((), None);
        let _ = delta;
        self.on_after_update.trigger((), None);
    }
}
