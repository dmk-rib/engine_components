use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::core::types::src::event_manager::EventControl;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type AsyncEventHandler<T> = Arc<dyn Fn(Option<T>) -> BoxFuture<'static, ()> + Send + Sync>;

struct AsyncEventState<T: Clone + Send + Sync + 'static> {
    enabled: bool,
    handlers: Vec<AsyncEventHandler<T>>,
}

impl<T: Clone + Send + Sync + 'static> AsyncEventState<T> {
    fn new() -> Self {
        Self {
            enabled: true,
            handlers: Vec::new(),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> EventControl for Mutex<AsyncEventState<T>> {
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
pub struct AsyncEvent<T: Clone + Send + Sync + 'static> {
    state: Arc<Mutex<AsyncEventState<T>>>,
}

impl<T: Clone + Send + Sync + 'static> Default for AsyncEvent<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Send + Sync + 'static> AsyncEvent<T> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(AsyncEventState::new())),
        }
    }

    pub fn add<F, Fut>(&self, handler: F)
    where
        F: Fn(Option<T>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut state = self.state.lock().expect("async event state lock poisoned");
        state
            .handlers
            .push(Arc::new(move |data| Box::pin(handler(data))));
    }

    pub fn remove(&self, handler: &AsyncEventHandler<T>) {
        let mut state = self.state.lock().expect("async event state lock poisoned");
        state.handlers.retain(|h| !Arc::ptr_eq(h, handler));
    }

    pub async fn trigger(&self, data: Option<T>) {
        let handlers = {
            let state = self.state.lock().expect("async event state lock poisoned");
            if !state.enabled {
                return;
            }
            state.handlers.clone()
        };
        for handler in handlers {
            handler(data.clone()).await;
        }
    }

    pub fn reset(&self) {
        let mut state = self.state.lock().expect("async event state lock poisoned");
        state.handlers.clear();
    }

    pub fn enabled(&self) -> bool {
        let state = self.state.lock().expect("async event state lock poisoned");
        state.enabled
    }

    pub fn control_handle(&self) -> Arc<dyn EventControl> {
        self.state.clone()
    }
}
