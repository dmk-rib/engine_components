use crate::core::types::data_map::DataMap;
use crate::core::types::event::Event;
use crate::core::types::interfaces::Disposable;
use crate::utils::uuid::{UuidError, UUID};
use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub type ComponentsHandle = Arc<Mutex<Components>>;

pub trait ComponentInstance: Send {
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
            list: DataMap::new(None),
            on_disposed: Event::new(),
            on_init: Event::new(),
            last_update: None,
        }))
    }

    pub fn add(
        &mut self,
        uuid: &str,
        instance: Arc<Mutex<dyn ComponentInstance>>,
    ) -> Result<(), UuidError> {
        if self.list.has(&uuid.to_string()) {
            return Err(UuidError {
                message: "You're trying to add a component that already exists in the components instance. Use Components.get() instead.".to_string(),
            });
        }
        UUID::validate(uuid)?;
        self.list.set(uuid.to_string(), instance);
        Ok(())
    }

    pub fn get<T: ComponentFactory>(
        &mut self,
        handle: ComponentsHandle,
    ) -> Arc<Mutex<dyn ComponentInstance>> {
        let uuid = T::UUID.to_string();
        if !self.list.has(&uuid) {
            let instance = Arc::new(Mutex::new(T::new(handle.clone())));
            let instance_dyn: Arc<Mutex<dyn ComponentInstance>> = instance;
            let _ = self.add(&uuid, Arc::clone(&instance_dyn));
            return instance_dyn;
        }
        self.list.get(&uuid).expect("component missing").clone()
    }

    pub fn get_component<T: ComponentFactory>(&self) -> Option<Arc<Mutex<dyn ComponentInstance>>> {
        self.list.get(&T::UUID.to_string()).cloned()
    }

    pub fn init(&mut self) {
        self.enabled = true;
        for (_, component) in self.list.iter() {
            if let Ok(mut component) = component.lock() {
                component.set_enabled(true);
            }
        }
        self.last_update = Some(Instant::now());
        self.update_once();
        self.on_init.trigger((), None);
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
        self.on_disposed.trigger((), None);
    }

    pub fn update_once(&mut self) {
        if !self.enabled {
            return;
        }
        let now = Instant::now();
        let delta = self
            .last_update
            .map(|last| (now - last).as_secs_f32())
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
}

impl Disposable for Components {
    fn dispose(&mut self) {
        Components::dispose(self);
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}
