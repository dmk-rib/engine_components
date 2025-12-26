use crate::core::types::event::Event;
use crate::core::types::interfaces::Disposable;
use bevy_math::Vec2;

pub struct Mouse {
    position: Vec2,
    raw_position: Vec2,
    pub on_disposed: Event<()>,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            raw_position: Vec2::ZERO,
            on_disposed: Event::new(),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn raw_position(&self) -> Vec2 {
        self.raw_position
    }

    pub fn update(&mut self, position: Vec2, raw_position: Vec2) {
        self.position = position;
        self.raw_position = raw_position;
    }
}

impl Disposable for Mouse {
    fn dispose(&mut self) {
        self.on_disposed.trigger((), None);
        self.on_disposed.reset();
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}
