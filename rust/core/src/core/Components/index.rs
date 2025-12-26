use crate::core::types::src::data_map::DataMap;
use crate::core::types::src::event::Event;
use crate::utils::uuid::{UuidError, UUID};
use std::any::Any;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub type ComponentsHandle = Arc<Mutex<Components>>;

pub trait ComponentInstance: Send + Sync {
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn is_disposable(&self) -> bool {
        false
    }
    fn dispose(&mut self) {}
    fn is_updateable(&self) -> bool {
        false
    }
    fn update(&mut self, _delta: f32) {}
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentFactory: ComponentInstance + Sized + 'static {
    const UUID: &'static str;
    fn new(components: ComponentsHandle) -> Self;
}

#[derive(Debug)]
pub enum ComponentsError {
    AlreadyExists(String),
    InvalidUuid(UuidError),
}

impl fmt::Display for ComponentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentsError::AlreadyExists(uuid) => {
                write!(f, "component with uuid {uuid} already exists")
            }
            ComponentsError::InvalidUuid(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for ComponentsError {}

impl From<UuidError> for ComponentsError {
    fn from(value: UuidError) -> Self {
        ComponentsError::InvalidUuid(value)
    }
}

pub struct Components {
    pub enabled: bool,
    pub list: DataMap<String, Arc<Mutex<dyn ComponentInstance>>>,
    pub on_disposed: Event<()>,
    pub on_init: Event<()>,
    last_update: Option<Instant>,
}

impl Components {
    pub const RELEASE: &'static str = "2.4.3";

    pub fn new() -> ComponentsHandle {
        Arc::new(Mutex::new(Self {
            enabled: false,
            list: DataMap::new(),
            on_disposed: Event::new(),
            on_init: Event::new(),
            last_update: None,
        }))
    }

    pub fn add(
        &mut self,
        uuid: &str,
        instance: Arc<Mutex<dyn ComponentInstance>>,
    ) -> Result<(), ComponentsError> {
        if self.list.contains_key(&uuid.to_string()) {
            return Err(ComponentsError::AlreadyExists(uuid.to_string()));
        }
        UUID::validate(uuid)?;
        self.list.set(uuid.to_string(), instance);
        Ok(())
    }

    pub fn get_by_uuid(&self, uuid: &str) -> Option<Arc<Mutex<dyn ComponentInstance>>> {
        self.list.get(&uuid.to_string()).cloned()
    }

    pub fn get_or_create<C: ComponentFactory>(
        &mut self,
        components: ComponentsHandle,
    ) -> Result<Arc<Mutex<dyn ComponentInstance>>, ComponentsError> {
        if let Some(existing) = self.list.get(&C::UUID.to_string()) {
            return Ok(existing.clone());
        }

        let instance = Arc::new(Mutex::new(C::new(components)));
        self.add(C::UUID, instance.clone())?;
        Ok(instance)
    }

    pub fn init(&mut self) {
        self.enabled = true;
        for (_, component) in self.list.iter() {
            if let Ok(mut component) = component.lock() {
                component.set_enabled(true);
            }
        }
        self.last_update = Some(Instant::now());
        self.update();
        self.on_init.trigger(Some(()));
    }

    pub fn update(&mut self) {
        if !self.enabled {
            return;
        }
        let now = Instant::now();
        let delta = self
            .last_update
            .map(|instant| (now - instant).as_secs_f32())
            .unwrap_or(0.0);
        self.last_update = Some(now);

        for (_, component) in self.list.iter() {
            if let Ok(mut component) = component.lock() {
                if component.enabled() && component.is_updateable() {
                    component.update(delta);
                }
            }
        }
    }

    pub fn dispose(&mut self) {
        self.enabled = false;
        for (_, component) in self.list.iter() {
            if let Ok(mut component) = component.lock() {
                component.set_enabled(false);
                if component.is_disposable() {
                    component.dispose();
                }
            }
        }
        self.on_disposed.trigger(Some(()));
    }
}
