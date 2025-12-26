use std::{collections::HashSet, hash::Hash, sync::Arc};

pub trait EventControl: Send + Sync {
    fn set_enabled(&self, active: bool);
    fn reset(&self);
}

#[derive(Clone)]
pub struct EventControlHandle(Arc<dyn EventControl>);

impl EventControlHandle {
    pub fn new(control: Arc<dyn EventControl>) -> Self {
        Self(control)
    }

    pub fn set_enabled(&self, active: bool) {
        self.0.set_enabled(active);
    }

    pub fn reset(&self) {
        self.0.reset();
    }
}

impl PartialEq for EventControlHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for EventControlHandle {}

impl Hash for EventControlHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let ptr = Arc::as_ptr(&self.0) as *const () as usize;
        ptr.hash(state);
    }
}

pub struct EventManager {
    list: HashSet<EventControlHandle>,
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            list: HashSet::new(),
        }
    }

    pub fn add<I>(&mut self, events: I)
    where
        I: IntoIterator<Item = EventControlHandle>,
    {
        for event in events {
            self.list.insert(event);
        }
    }

    pub fn remove<I>(&mut self, events: I)
    where
        I: IntoIterator<Item = EventControlHandle>,
    {
        for event in events {
            self.list.remove(&event);
        }
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
