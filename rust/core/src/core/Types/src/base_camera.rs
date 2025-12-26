use crate::core::types::base_world_item::BaseWorldItem;
use crate::core::types::interfaces::CameraControls;
use bevy_math::Vec3;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
}

pub struct BaseCamera {
    pub base: BaseWorldItem,
    pub enabled: bool,
    pub camera: Camera,
    pub controls: Option<CameraControls>,
}

impl BaseCamera {
    pub fn has_camera_controls(&self) -> bool {
        self.controls.is_some()
    }
}
