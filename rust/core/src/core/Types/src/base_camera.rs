use crate::core::types::src::base_world_item::BaseWorldItem;
use crate::core::types::src::interfaces::CameraControls;

#[derive(Debug, Clone, Default)]
pub struct Camera;

pub struct BaseCamera {
    pub base: BaseWorldItem,
    pub enabled: bool,
    pub camera: Camera,
    pub controls: Option<CameraControls>,
}

pub type BaseCameraHandle = std::sync::Arc<std::sync::Mutex<BaseCamera>>;

impl BaseCamera {
    pub fn has_camera_controls(&self) -> bool {
        self.controls.is_some()
    }
}
