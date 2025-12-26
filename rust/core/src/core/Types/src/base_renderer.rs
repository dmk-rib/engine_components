use crate::core::types::base_world_item::BaseWorldItem;
use crate::core::types::event::Event;
use bevy_math::Vec2;
use bevy_math::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
    pub is_local: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Renderer {
    pub clipping_planes: Vec<Plane>,
    pub size: Vec2,
}

pub struct BaseRenderer {
    pub base: BaseWorldItem,
    pub three: Renderer,
    pub on_after_update: Event<()>,
    pub on_before_update: Event<()>,
    pub on_disposed: Event<()>,
    pub on_resize: Event<Vec2>,
    pub on_clipping_planes_updated: Event<()>,
    pub clipping_planes: Vec<Plane>,
}

impl BaseRenderer {
    pub fn new(base: BaseWorldItem, renderer: Renderer) -> Self {
        Self {
            base,
            three: renderer,
            on_after_update: Event::new(),
            on_before_update: Event::new(),
            on_disposed: Event::new(),
            on_resize: Event::new(),
            on_clipping_planes_updated: Event::new(),
            clipping_planes: Vec::new(),
        }
    }

    pub fn update(&mut self, _delta: Option<f32>) {}

    pub fn dispose(&mut self) {
        self.on_disposed.trigger((), None);
        self.on_disposed.reset();
    }

    pub fn get_size(&self) -> Vec2 {
        self.three.size
    }

    pub fn resize(&mut self, size: Vec2) {
        self.three.size = size;
        self.on_resize.trigger(size, None);
    }

    pub fn update_clipping_planes(&self) {
        self.on_clipping_planes_updated.trigger((), None);
    }

    pub fn set_plane(&mut self, active: bool, mut plane: Plane, is_local: Option<bool>) {
        plane.is_local = is_local.unwrap_or(false);

        let index = self
            .clipping_planes
            .iter()
            .position(|entry| entry.normal == plane.normal && entry.distance == plane.distance);
        match (active, index) {
            (true, None) => self.clipping_planes.push(plane),
            (false, Some(idx)) => {
                self.clipping_planes.remove(idx);
            }
            _ => {}
        }

        self.three.clipping_planes = self
            .clipping_planes
            .iter()
            .copied()
            .filter(|entry| !entry.is_local)
            .collect();
    }
}
