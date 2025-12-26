use std::sync::Arc;

pub type EventHandler<T> = Arc<dyn Fn(&T) + Send + Sync>;

#[derive(Default)]
pub struct Event<T> {
    pub enabled: bool,
    handlers: Vec<EventHandler<T>>,
}

impl<T> Event<T> {
    pub fn new() -> Self {
        Self {
            enabled: true,
            handlers: Vec::new(),
        }
    }

    pub fn add(&mut self, handler: EventHandler<T>) {
        self.handlers.push(handler);
    }

    pub fn remove(&mut self, handler: &EventHandler<T>) {
        self.handlers.retain(|h| !Arc::ptr_eq(h, handler));
    }

    pub fn trigger(&self, data: T, active_override: Option<bool>) {
        if !active_override.unwrap_or(self.enabled) {
            return;
        }
        for handler in &self.handlers {
            handler(&data);
        }
    }

    pub fn reset(&mut self) {
        self.handlers.clear();
    }
}
