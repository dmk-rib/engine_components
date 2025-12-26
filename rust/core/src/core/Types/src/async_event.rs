use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type AsyncEventHandler<T> =
    Arc<dyn Fn(&T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[derive(Default)]
pub struct AsyncEvent<T> {
    pub enabled: bool,
    handlers: Vec<AsyncEventHandler<T>>,
}

impl<T> AsyncEvent<T> {
    pub fn new() -> Self {
        Self {
            enabled: true,
            handlers: Vec::new(),
        }
    }

    pub fn add(&mut self, handler: AsyncEventHandler<T>) {
        self.handlers.push(handler);
    }

    pub fn remove(&mut self, handler: &AsyncEventHandler<T>) {
        self.handlers.retain(|h| !Arc::ptr_eq(h, handler));
    }

    pub async fn trigger(&self, data: T, active_override: Option<bool>) {
        if !active_override.unwrap_or(self.enabled) {
            return;
        }
        for handler in &self.handlers {
            handler(&data).await;
        }
    }

    pub fn reset(&mut self) {
        self.handlers.clear();
    }
}
