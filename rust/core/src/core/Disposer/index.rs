use crate::core::components::{ComponentFactory, ComponentInstance, ComponentsHandle};
use crate::core::types::base_scene::{Geometry, Material, Mesh, Object3d};
use crate::core::types::component::Component;
use crate::core::types::event::Event;
use crate::core::types::interfaces::Disposable;
use std::any::Any;
use std::collections::BTreeSet;

pub struct Disposer {
    pub component: Component,
    disposed_components: BTreeSet<String>,
    pub on_disposed: Event<()>,
}

impl Disposer {
    pub const UUID: &'static str = "76e9cd8e-ad8f-4753-9ef6-cbc60f7247fe";

    pub fn get_disposed_components(&self) -> &BTreeSet<String> {
        &self.disposed_components
    }

    pub fn destroy(&mut self, object: &mut Object3d, materials: bool, recursive: bool) {
        for child in &mut object.children {
            self.dispose_mesh(child, materials, recursive);
        }
        object.children.clear();
    }

    pub fn dispose_geometry(&mut self, geometry: &mut Geometry) {
        geometry.dispose();
    }

    fn dispose_mesh(&mut self, mesh: &mut Mesh, materials: bool, recursive: bool) {
        if let Some(mut geometry) = mesh.geometry.take() {
            self.dispose_geometry(&mut geometry);
        }
        if materials {
            Self::dispose_materials(&mut mesh.materials);
        }
        if recursive {
            for child in &mut mesh.children {
                self.dispose_mesh(child, materials, recursive);
            }
        }
        mesh.children.clear();
    }

    fn dispose_materials(materials: &mut Vec<Material>) {
        for material in materials.iter_mut() {
            material.dispose();
        }
        materials.clear();
    }
}

impl ComponentFactory for Disposer {
    const UUID: &'static str = Disposer::UUID;

    fn new(components: ComponentsHandle) -> Self {
        Self {
            component: Component::new(components),
            disposed_components: BTreeSet::new(),
            on_disposed: Event::new(),
        }
    }
}

impl ComponentInstance for Disposer {
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
        self.disposed_components.insert(Self::UUID.to_string());
        self.on_disposed.trigger((), None);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Disposable for Disposer {
    fn dispose(&mut self) {
        self.on_disposed.trigger((), None);
        self.on_disposed.reset();
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}
