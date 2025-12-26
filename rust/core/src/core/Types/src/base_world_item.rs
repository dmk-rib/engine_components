use crate::core::components::ComponentsHandle;
use crate::core::types::src::base::Base;
use crate::core::types::src::data_map::DataMap;
use crate::core::types::src::event::Event;
use crate::core::types::src::world::WorldHandle;

#[derive(Clone)]
pub struct WorldChange {
    pub world: WorldHandle,
    pub action: WorldAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldAction {
    Added,
    Removed,
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
            worlds: DataMap::new(),
            on_world_changed: Event::new(),
            current_world: None,
        }
    }

    pub fn set_current_world(&mut self, value: Option<WorldHandle>) {
        self.current_world = value;
    }

    pub fn current_world(&self) -> Option<&WorldHandle> {
        self.current_world.as_ref()
    }

    pub fn handle_world_change(&mut self, change: WorldChange) {
        if change.action == WorldAction::Removed {
            if let Ok(world) = change.world.lock() {
                self.worlds.delete(&world.uuid);
            }
        }
        self.on_world_changed.trigger(Some(change));
    }

    pub fn add_world(&mut self, world: WorldHandle) {
        if let Ok(locked) = world.lock() {
            self.worlds.set(locked.uuid.clone(), world.clone());
        }
        self.handle_world_change(WorldChange {
            world,
            action: WorldAction::Added,
        });
    }

    pub fn remove_world(&mut self, world: WorldHandle) {
        self.handle_world_change(WorldChange {
            world,
            action: WorldAction::Removed,
        });
    }
}
