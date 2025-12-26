use std::sync::{Arc, Mutex};

use crate::core::types::src::event_manager::EventControl;

pub type EventHandler<T> = Arc<dyn Fn(Option<T>) + Send + Sync>;

struct EventState<T: Clone + Send + Sync + 'static> {
    enabled: bool,
    handlers: Vec<EventHandler<T>>,
}

impl<T: Clone + Send + Sync + 'static> EventState<T> {
    fn new() -> Self {
        Self {
            enabled: true,
            handlers: Vec::new(),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> EventControl for Mutex<EventState<T>> {
    fn set_enabled(&self, active: bool) {
        if let Ok(mut state) = self.lock() {
            state.enabled = active;
        }
    }

    fn reset(&self) {
        if let Ok(mut state) = self.lock() {
            state.handlers.clear();
        }
    }
}

#[derive(Clone)]
pub struct Event<T: Clone + Send + Sync + 'static> {
    state: Arc<Mutex<EventState<T>>>,
}

impl<T: Clone + Send + Sync + 'static> Default for Event<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Send + Sync + 'static> Event<T> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(EventState::new())),
        }
    }

    pub fn add(&self, handler: EventHandler<T>) {
        let mut state = self.state.lock().expect("event state lock poisoned");
        state.handlers.push(handler);
    }

    pub fn remove(&self, handler: &EventHandler<T>) {
        let mut state = self.state.lock().expect("event state lock poisoned");
        state.handlers.retain(|h| !Arc::ptr_eq(h, handler));
    }

    pub fn trigger(&self, data: Option<T>) {
        let handlers = {
            let state = self.state.lock().expect("event state lock poisoned");
            if !state.enabled {
                return;
            }
            state.handlers.clone()
        };
        for handler in handlers {
            handler(data.clone());
        }
    }

    pub fn reset(&self) {
        let mut state = self.state.lock().expect("event state lock poisoned");
        state.handlers.clear();
    }

    pub fn enabled(&self) -> bool {
        let state = self.state.lock().expect("event state lock poisoned");
        state.enabled
    }

    pub fn control_handle(&self) -> Arc<dyn EventControl> {
        self.state.clone()
    }
}
