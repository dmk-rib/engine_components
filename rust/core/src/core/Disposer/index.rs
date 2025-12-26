use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use crate::core::components::{
    ComponentFactory, ComponentInstance, ComponentsError, ComponentsHandle,
};
use crate::core::types::src::base_scene::{Geometry, Material, Mesh};
use std::any::Any;

pub struct Disposer {
    enabled: bool,
    disposed_components: BTreeSet<String>,
}

impl Disposer {
    pub const UUID: &'static str = "76e9cd8e-ad8f-4753-9ef6-cbc60f7247fe";

    pub fn new(
        components: ComponentsHandle,
    ) -> Result<Arc<Mutex<dyn ComponentInstance>>, ComponentsError> {
        let instance = Arc::new(Mutex::new(Self {
            enabled: true,
            disposed_components: BTreeSet::new(),
        }));
        components
            .lock()
            .expect("components lock poisoned")
            .add(Self::UUID, instance.clone())?;
        Ok(instance)
    }

    pub fn get(&self) -> &BTreeSet<String> {
        &self.disposed_components
    }

    pub fn destroy(&mut self, object: &mut Mesh) {
        object.remove_from_parent();
        self.dispose_geometry_and_materials(object, true);
        if !object.children.is_empty() {
            self.dispose_children(object);
        }
        object.children.clear();
    }

    pub fn dispose_geometry(&self, geometry: &mut Geometry) {
        geometry.dispose();
    }

    fn dispose_geometry_and_materials(&self, mesh: &mut Mesh, materials: bool) {
        if let Some(mut geometry) = mesh.geometry.take() {
            self.dispose_geometry(&mut geometry);
        }
        if materials {
            Disposer::dispose_materials(&mut mesh.material);
        }
        mesh.material.clear();
    }

    fn dispose_children(&mut self, mesh: &mut Mesh) {
        for child in &mut mesh.children {
            self.destroy(child);
        }
    }

    fn dispose_materials(materials: &mut [Material]) {
        for material in materials.iter_mut() {
            material.dispose();
        }
    }
}

impl ComponentInstance for Disposer {
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn is_disposable(&self) -> bool {
        true
    }

    fn dispose(&mut self) {
        self.disposed_components.insert(Self::UUID.to_string());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ComponentFactory for Disposer {
    const UUID: &'static str = Disposer::UUID;

    fn new(_components: ComponentsHandle) -> Self {
        Self {
            enabled: true,
            disposed_components: BTreeSet::new(),
        }
    }
}
