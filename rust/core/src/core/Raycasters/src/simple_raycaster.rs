use crate::core::components::ComponentsHandle;
use crate::core::raycasters::mouse::Mouse;
use crate::core::types::event::Event;
use crate::core::types::interfaces::Disposable;
use crate::core::types::world::WorldHandle;
use bevy_math::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct RaycastHit {
    pub point: Vec3,
    pub distance: f32,
}

pub struct SimpleRaycaster {
    pub enabled: bool,
    pub components: ComponentsHandle,
    pub on_disposed: Event<()>,
    pub mouse: Mouse,
    pub world: WorldHandle,
}

impl SimpleRaycaster {
    pub fn new(components: ComponentsHandle, world: WorldHandle) -> Self {
        Self {
            enabled: true,
            components,
            on_disposed: Event::new(),
            mouse: Mouse::new(),
            world,
        }
    }

    pub fn cast_ray_to_objects(
        &self,
        _items: Option<&[()]>,
        _position: Option<Vec2>,
    ) -> Option<RaycastHit> {
        None
    }

    pub fn cast_ray(&self, _data: Option<RaycastRequest>) -> Option<RaycastHit> {
        None
    }

    pub fn cast_ray_from_vector(
        &self,
        _origin: Vec3,
        _direction: Vec3,
        _items: Option<&[()]>,
    ) -> Option<RaycastHit> {
        None
    }
}

impl Disposable for SimpleRaycaster {
    fn dispose(&mut self) {
        self.mouse.dispose();
        self.on_disposed.trigger((), None);
        self.on_disposed.reset();
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}

pub struct RaycastRequest {
    pub snapping_classes: Option<Vec<String>>,
    pub position: Option<Vec2>,
}
