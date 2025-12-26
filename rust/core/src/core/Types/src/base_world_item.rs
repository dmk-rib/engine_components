use crate::core::components::ComponentsHandle;
use crate::core::types::base::Base;
use crate::core::types::data_map::DataMap;
use crate::core::types::event::Event;
use crate::core::types::world::WorldHandle;

#[derive(Clone)]
pub enum WorldAction {
    Added,
    Removed,
}

#[derive(Clone)]
pub struct WorldChange {
    pub world: WorldHandle,
    pub action: WorldAction,
}

pub struct BaseWorldItem {
    pub base: Base,
    pub worlds: DataMap<String, WorldHandle>,
    pub on_world_changed: Event<WorldChange>,
    current_world: Option<WorldHandle>,
}

impl BaseWorldItem {
    pub fn new(components: ComponentsHandle) -> Self {
        Self {
            base: Base::new(components),
            worlds: DataMap::new(None),
            on_world_changed: Event::new(),
            current_world: None,
        }
    }

    pub fn set_current_world(&mut self, value: Option<WorldHandle>) {
        self.current_world = value;
    }

    pub fn current_world(&self) -> Option<WorldHandle> {
        self.current_world.clone()
    }

    pub fn add_world(&mut self, world: WorldHandle) {
        if let Ok(world_locked) = world.lock() {
            let uuid = world_locked.uuid.clone();
            self.worlds.set(uuid.clone(), world.clone());
        }
        self.on_world_changed.trigger(
            WorldChange {
                world,
                action: WorldAction::Added,
            },
            None,
        );
    }

    pub fn remove_world(&mut self, world: &WorldHandle) {
        if let Ok(world_locked) = world.lock() {
            self.worlds.delete(&world_locked.uuid);
        }
        self.on_world_changed.trigger(
            WorldChange {
                world: world.clone(),
                action: WorldAction::Removed,
            },
            None,
        );
    }
}
