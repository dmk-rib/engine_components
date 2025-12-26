use crate::core::components::{ComponentFactory, ComponentInstance, ComponentsHandle};
use crate::core::types::component::Component;
use crate::core::types::event::Event;
use crate::core::types::interfaces::Disposable;
use crate::core::types::world::WorldHandle;
use std::any::Any;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[path = "src/index.rs"]
pub mod src;

pub use src::*;

pub struct Raycasters {
    pub component: Component,
    pub list: BTreeMap<String, Arc<Mutex<src::SimpleRaycaster>>>,
    pub on_disposed: Event<()>,
}

impl Raycasters {
    pub const UUID: &'static str = "d5d8bdf0-db25-4952-b951-b643af207ace";

    pub fn get(&mut self, world: WorldHandle) -> Arc<Mutex<src::SimpleRaycaster>> {
        if let Ok(world_locked) = world.lock() {
            let uuid = world_locked.uuid.clone();
            if let Some(raycaster) = self.list.get(&uuid) {
                return Arc::clone(raycaster);
            }
            let raycaster = Arc::new(Mutex::new(src::SimpleRaycaster::new(
                self.component.base.components.clone(),
                world.clone(),
            )));
            self.list.insert(uuid, Arc::clone(&raycaster));
            return raycaster;
        }
        Arc::new(Mutex::new(src::SimpleRaycaster::new(
            self.component.base.components.clone(),
            world,
        )))
    }

    pub fn delete(&mut self, world: &WorldHandle) {
        if let Ok(world_locked) = world.lock() {
            self.list.remove(&world_locked.uuid);
        }
    }
}

impl ComponentFactory for Raycasters {
    const UUID: &'static str = Raycasters::UUID;

    fn new(components: ComponentsHandle) -> Self {
        Self {
            component: Component::new(components),
            list: BTreeMap::new(),
            on_disposed: Event::new(),
        }
    }
}

impl ComponentInstance for Raycasters {
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
        for (_, raycaster) in self.list.iter() {
            if let Ok(mut raycaster) = raycaster.lock() {
                raycaster.dispose();
            }
        }
        self.list.clear();
        self.on_disposed.trigger((), None);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Disposable for Raycasters {
    fn dispose(&mut self) {
        for (_, raycaster) in self.list.iter() {
            if let Ok(mut raycaster) = raycaster.lock() {
                raycaster.dispose();
            }
        }
        self.list.clear();
        self.on_disposed.trigger((), None);
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}
