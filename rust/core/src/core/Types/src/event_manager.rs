use crate::core::types::async_event::AsyncEvent;
use crate::core::types::event::Event;
use std::sync::{Arc, Mutex};

pub trait EventControl: Send + Sync {
    fn set_enabled(&self, active: bool);
    fn reset(&self);
}

impl<T: Send + Sync> EventControl for Arc<Mutex<Event<T>>> {
    fn set_enabled(&self, active: bool) {
        if let Ok(mut event) = self.lock() {
            event.enabled = active;
        }
    }

    fn reset(&self) {
        if let Ok(mut event) = self.lock() {
            event.reset();
        }
    }
}

impl<T: Send + Sync> EventControl for Arc<Mutex<AsyncEvent<T>>> {
    fn set_enabled(&self, active: bool) {
        if let Ok(mut event) = self.lock() {
            event.enabled = active;
        }
    }

    fn reset(&self) {
        if let Ok(mut event) = self.lock() {
            event.reset();
        }
    }
}

#[derive(Default)]
pub struct EventManager {
    list: Vec<Arc<dyn EventControl>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add<I>(&mut self, events: I)
    where
        I: IntoIterator<Item = Arc<dyn EventControl>>,
    {
        for event in events {
            self.list.push(event);
        }
    }

    pub fn remove(&mut self, events: &[Arc<dyn EventControl>]) {
        self.list
            .retain(|event| !events.iter().any(|item| Arc::ptr_eq(event, item)));
    }

    pub fn set(&self, active: bool) {
        for event in &self.list {
            event.set_enabled(active);
        }
    }

    pub fn reset(&self) {
        for event in &self.list {
            event.reset();
        }
    }
}
