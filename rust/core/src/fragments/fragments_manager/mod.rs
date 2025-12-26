#[path = "../FragmentsManager/src/types.rs"]
pub mod types;

use crate::core::components::{ComponentFactory, ComponentInstance, ComponentsHandle};
use crate::core::types::component::Component;
use crate::core::types::data_map::DataMap;
use crate::core::types::event::Event;
use crate::core::types::interfaces::Disposable;
use crate::utils::uuid::{UuidError, UUID};
use bevy_math::Vec3;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct FragmentModel {
    items_volume: BTreeMap<u32, f64>,
}

impl FragmentModel {
    pub fn new(items_volume: BTreeMap<u32, f64>) -> Self {
        Self { items_volume }
    }

    pub async fn get_items_volume(&self, local_ids: &[u32]) -> f64 {
        local_ids
            .iter()
            .filter_map(|id| self.items_volume.get(id))
            .sum()
    }
}

pub struct FragmentsManager {
    pub component: Component,
    pub list: DataMap<String, FragmentModel>,
    pub initialized: bool,
    pub on_disposed: Event<()>,
}

impl FragmentsManager {
    pub const UUID: &'static str = "fragments-manager";

    pub fn new(components: ComponentsHandle) -> Self {
        Self {
            component: Component::new(components),
            list: DataMap::new(None),
            initialized: false,
            on_disposed: Event::new(),
        }
    }

    pub fn add_model(&mut self, model_id: &str, model: FragmentModel) -> Result<(), UuidError> {
        UUID::validate(model_id)?;
        self.list.set(model_id.to_string(), model);
        Ok(())
    }

    pub async fn raycast(
        &self,
        _camera_position: Vec3,
        _mouse_position: Vec3,
    ) -> Option<RaycastHit> {
        None
    }
}

impl Disposable for FragmentsManager {
    fn dispose(&mut self) {
        self.list.dispose();
        self.on_disposed.trigger((), None);
        self.on_disposed.reset();
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}

impl ComponentFactory for FragmentsManager {
    const UUID: &'static str = FragmentsManager::UUID;

    fn new(components: ComponentsHandle) -> Self {
        FragmentsManager::new(components)
    }
}

impl ComponentInstance for FragmentsManager {
    fn enabled(&self) -> bool {
        self.component.enabled()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.component.set_enabled(enabled);
    }

    fn is_disposable(&self) -> bool {
        true
    }

    fn dispose(&mut self) {
        Disposable::dispose(self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct RaycastHit {
    pub point: Vec3,
    pub distance: f32,
}
