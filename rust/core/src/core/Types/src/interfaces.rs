use bevy_math::Vec2;

use crate::core::types::src::event::Event;
use crate::core::types::src::event_manager::EventManager;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(std::collections::BTreeMap<String, JsonValue>),
}

pub trait Disposable {
    fn dispose(&mut self);
    fn on_disposed(&self) -> &Event<()>;
}

pub trait Hideable {
    fn visible(&self) -> bool;
    fn set_visible(&mut self, value: bool);
}

pub trait Resizeable {
    fn resize(&mut self, size: Option<Vec2>);
    fn on_resize(&self) -> &Event<Vec2>;
    fn get_size(&self) -> Vec2;
}

pub trait Updateable {
    fn on_after_update(&self) -> &Event<()>;
    fn on_before_update(&self) -> &Event<()>;
    fn update(&mut self, delta: Option<f32>);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progress {
    pub current: usize,
    pub total: usize,
}

pub trait Createable {
    fn create(&mut self, data: JsonValue);
    fn end_creation(&mut self, _data: Option<JsonValue>) {}
    fn cancel_creation(&mut self, _data: Option<JsonValue>) {}
    fn delete(&mut self, data: JsonValue);
}

pub trait Configurable<T, U> {
    fn is_setup(&self) -> bool;
    fn setup(&mut self, config: Option<U>);
    fn on_setup(&self) -> &Event<()>;
    fn config(&self) -> &T;
    fn config_mut(&mut self) -> &mut T;
}

pub trait CameraControllable {
    fn controls(&self) -> &CameraControls;
}

pub trait Eventable {
    fn event_manager(&self) -> &EventManager;
}

pub trait WithUi {
    fn ui(&self) -> &std::collections::BTreeMap<String, Box<dyn Fn() -> String + Send + Sync>>;
}

#[derive(Debug, Clone, Default)]
pub struct SerializationResult<D = JsonValue, S = JsonValue> {
    pub data: Option<Vec<D>>,
    pub settings: Option<S>,
}

pub trait Serializable<D = JsonValue, S = JsonValue> {
    fn import(&mut self, result: SerializationResult<D, S>, args: Vec<JsonValue>);
    fn export(&self, args: Vec<JsonValue>) -> SerializationResult<D, S>;
}

#[derive(Debug, Clone, Default)]
pub struct CameraControls;
