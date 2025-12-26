use crate::core::types::event::Event;
use std::collections::BTreeSet;

pub struct DataSet<T: Ord + Clone> {
    set: BTreeSet<T>,
    pub on_item_added: Event<T>,
    pub on_item_deleted: Event<()>,
    pub on_cleared: Event<()>,
    pub guard: Box<dyn Fn(&T) -> bool + Send + Sync>,
}

impl<T: Ord + Clone> DataSet<T> {
    pub fn new(iterable: Option<Vec<T>>) -> Self {
        let mut set = BTreeSet::new();
        if let Some(values) = iterable {
            for value in values {
                set.insert(value);
            }
        }
        Self {
            set,
            on_item_added: Event::new(),
            on_item_deleted: Event::new(),
            on_cleared: Event::new(),
            guard: Box::new(|_| true),
        }
    }

    pub fn clear(&mut self) {
        self.set.clear();
        self.on_cleared.trigger((), None);
    }

    pub fn add<I>(&mut self, values: I)
    where
        I: IntoIterator<Item = T>,
    {
        for value in values {
            if self.set.contains(&value) {
                continue;
            }
            if !(self.guard)(&value) {
                continue;
            }
            self.set.insert(value.clone());
            self.on_item_added.trigger(value, None);
        }
    }

    pub fn delete(&mut self, value: &T) -> bool {
        let deleted = self.set.remove(value);
        if deleted {
            self.on_item_deleted.trigger((), None);
        }
        deleted
    }

    pub fn dispose(&mut self) {
        self.clear();
        self.on_item_added.reset();
        self.on_item_deleted.reset();
        self.on_cleared.reset();
    }

    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.set.iter()
    }
}
