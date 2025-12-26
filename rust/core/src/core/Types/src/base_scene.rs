use crate::core::types::base_world_item::BaseWorldItem;
use crate::core::types::event::Event;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct Geometry {
    pub disposed: bool,
}

impl Geometry {
    pub fn dispose(&mut self) {
        self.disposed = true;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Material {
    pub disposed: bool,
}

impl Material {
    pub fn dispose(&mut self) {
        self.disposed = true;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Mesh {
    pub geometry: Option<Geometry>,
    pub materials: Vec<Material>,
    pub children: Vec<Mesh>,
}

#[derive(Debug, Clone, Default)]
pub struct Object3d {
    pub children: Vec<Mesh>,
}

impl Object3d {
    pub fn clear(&mut self) {
        self.children.clear();
    }
}

#[derive(Debug, Clone, Default)]
pub struct DirectionalLight {
    pub disposed: bool,
    pub target: Object3d,
}

impl DirectionalLight {
    pub fn dispose(&mut self) {
        self.disposed = true;
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmbientLight {
    pub disposed: bool,
}

impl AmbientLight {
    pub fn dispose(&mut self) {
        self.disposed = true;
    }
}

pub struct BaseScene {
    pub base: BaseWorldItem,
    pub on_disposed: Event<()>,
    pub three: Object3d,
    pub directional_lights: BTreeMap<String, DirectionalLight>,
    pub ambient_lights: BTreeMap<String, AmbientLight>,
}

impl BaseScene {
    pub fn new(base: BaseWorldItem, three: Object3d) -> Self {
        Self {
            base,
            on_disposed: Event::new(),
            three,
            directional_lights: BTreeMap::new(),
            ambient_lights: BTreeMap::new(),
        }
    }

    pub fn dispose(&mut self) {
        self.delete_all_lights();
        self.three.clear();
        self.on_disposed.trigger((), None);
        self.on_disposed.reset();
    }

    pub fn delete_all_lights(&mut self) {
        for (_, light) in self.directional_lights.iter_mut() {
            light.dispose();
        }
        self.directional_lights.clear();
        for (_, light) in self.ambient_lights.iter_mut() {
            light.dispose();
        }
        self.ambient_lights.clear();
    }
}
