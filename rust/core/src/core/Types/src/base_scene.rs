use std::collections::BTreeMap;

use crate::core::components::ComponentsHandle;
use crate::core::disposer::Disposer;
use crate::core::types::src::base_world_item::BaseWorldItem;
use crate::core::types::src::event::Event;
use crate::core::types::src::interfaces::Disposable;

#[derive(Debug, Clone, Default)]
pub struct Geometry;

impl Geometry {
    pub fn dispose(&mut self) {}
}

#[derive(Debug, Clone, Default)]
pub struct Material;

impl Material {
    pub fn dispose(&mut self) {}
}

#[derive(Debug, Clone, Default)]
pub struct Object3d;

#[derive(Debug, Clone, Default)]
pub struct Mesh {
    pub children: Vec<Mesh>,
    pub geometry: Option<Geometry>,
    pub material: Vec<Material>,
}

impl Mesh {
    pub fn remove_from_parent(&mut self) {}
}

#[derive(Debug, Clone, Default)]
pub struct Scene {
    pub children: Vec<Mesh>,
}

#[derive(Debug, Clone, Default)]
pub struct DirectionalLight {
    pub target: Mesh,
}

impl DirectionalLight {
    pub fn remove_from_parent(&mut self) {}
    pub fn dispose(&mut self) {}
}

#[derive(Debug, Clone, Default)]
pub struct AmbientLight;

impl AmbientLight {
    pub fn remove_from_parent(&mut self) {}
    pub fn dispose(&mut self) {}
}

pub struct BaseScene {
    pub base: BaseWorldItem,
    pub on_disposed: Event<()>,
    pub scene: Scene,
    pub directional_lights: BTreeMap<String, DirectionalLight>,
    pub ambient_lights: BTreeMap<String, AmbientLight>,
}

pub type BaseSceneHandle = std::sync::Arc<std::sync::Mutex<BaseScene>>;

impl BaseScene {
    pub fn new(components: ComponentsHandle) -> Self {
        Self {
            base: BaseWorldItem::new(components),
            on_disposed: Event::new(),
            scene: Scene::default(),
            directional_lights: BTreeMap::new(),
            ambient_lights: BTreeMap::new(),
        }
    }

    pub fn delete_all_lights(&mut self) {
        let directional = std::mem::take(&mut self.directional_lights);
        for (_, mut light) in directional {
            light.remove_from_parent();
            light.target.remove_from_parent();
            light.dispose();
        }
        let ambient = std::mem::take(&mut self.ambient_lights);
        for (_, mut light) in ambient {
            light.remove_from_parent();
            light.dispose();
        }
    }
}

impl Disposable for BaseScene {
    fn dispose(&mut self) {
        if let Ok(components) = self.base.base.components.lock() {
            if let Some(component) = components.get_by_uuid(Disposer::UUID) {
                if let Ok(mut component) = component.lock() {
                    if let Some(disposer) = component.as_any_mut().downcast_mut::<Disposer>() {
                        for child in &mut self.scene.children {
                            disposer.destroy(child);
                        }
                    }
                }
            }
        }

        self.delete_all_lights();
        self.scene.children.clear();
        self.on_disposed.trigger(Some(()));
        self.on_disposed.reset();
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}
