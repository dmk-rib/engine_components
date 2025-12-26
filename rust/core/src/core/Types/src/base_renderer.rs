use bevy_math::{Vec2, Vec3};

use crate::core::types::src::base_world_item::BaseWorldItem;
use crate::core::types::src::event::Event;
use crate::core::types::src::interfaces::{Disposable, Resizeable, Updateable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub normal: Vec3,
    pub constant: f32,
    pub is_local: bool,
}

impl Plane {
    pub fn new(normal: Vec3, constant: f32) -> Self {
        Self {
            normal,
            constant,
            is_local: false,
        }
    }
}

#[derive(Default)]
pub struct Renderer {
    pub clipping_planes: Vec<Plane>,
}

pub struct BaseRenderer {
    pub base: BaseWorldItem,
    pub renderer: Renderer,
    pub on_after_update: Event<()>,
    pub on_before_update: Event<()>,
    pub on_disposed: Event<()>,
    pub on_resize: Event<Vec2>,
    pub on_clipping_planes_updated: Event<()>,
    pub clipping_planes: Vec<Plane>,
}

pub type BaseRendererHandle = std::sync::Arc<std::sync::Mutex<BaseRenderer>>;

impl BaseRenderer {
    pub fn new(base: BaseWorldItem) -> Self {
        Self {
            base,
            renderer: Renderer::default(),
            on_after_update: Event::new(),
            on_before_update: Event::new(),
            on_disposed: Event::new(),
            on_resize: Event::new(),
            on_clipping_planes_updated: Event::new(),
            clipping_planes: Vec::new(),
        }
    }

    pub fn update_clipping_planes(&self) {
        self.on_clipping_planes_updated.trigger(Some(()));
    }

    pub fn set_plane(&mut self, active: bool, mut plane: Plane, is_local: Option<bool>) {
        plane.is_local = is_local.unwrap_or(false);
        if active {
            if !self.clipping_planes.iter().any(|p| *p == plane) {
                self.clipping_planes.push(plane);
            }
        } else {
            self.clipping_planes.retain(|p| *p != plane);
        }
        self.renderer.clipping_planes = self
            .clipping_planes
            .iter()
            .copied()
            .filter(|p| !p.is_local)
            .collect();
    }
}

impl Updateable for BaseRenderer {
    fn on_after_update(&self) -> &Event<()> {
        &self.on_after_update
    }

    fn on_before_update(&self) -> &Event<()> {
        &self.on_before_update
    }

    fn update(&mut self, _delta: Option<f32>) {}
}

impl Disposable for BaseRenderer {
    fn dispose(&mut self) {
        self.on_disposed.trigger(Some(()));
        self.on_disposed.reset();
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}

impl Resizeable for BaseRenderer {
    fn resize(&mut self, size: Option<Vec2>) {
        if let Some(size) = size {
            self.on_resize.trigger(Some(size));
        }
    }

    fn on_resize(&self) -> &Event<Vec2> {
        &self.on_resize
    }

    fn get_size(&self) -> Vec2 {
        Vec2::ZERO
    }
}
