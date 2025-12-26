use crate::core::types::src::base_camera::BaseCameraHandle;
use crate::core::types::src::base_renderer::BaseRendererHandle;
use crate::core::types::src::base_scene::{BaseSceneHandle, Mesh};
use crate::core::types::src::event::Event;

#[derive(Clone)]
pub struct World {
    pub meshes: Vec<Mesh>,
    pub scene: BaseSceneHandle,
    pub default_camera: BaseCameraHandle,
    pub camera: BaseCameraHandle,
    pub on_camera_changed: Event<()>,
    pub renderer: Option<BaseRendererHandle>,
    pub uuid: String,
    pub is_disposing: bool,
}

impl World {
    pub fn use_default_camera(&mut self) {
        self.camera = self.default_camera.clone();
        self.on_camera_changed.trigger(Some(()));
    }
}

pub type WorldHandle = std::sync::Arc<std::sync::Mutex<World>>;
