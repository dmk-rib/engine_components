use std::collections::BTreeSet;

use crate::core::types::src::event::Event;

pub struct DataSet<T>
where
    T: Ord + Clone + Send + Sync + 'static,
{
    set: BTreeSet<T>,
    pub on_item_added: Event<T>,
    pub on_item_deleted: Event<()>,
    pub on_cleared: Event<()>,
    pub guard: Box<dyn Fn(&T) -> bool + Send + Sync>,
}

impl<T> Default for DataSet<T>
where
    T: Ord + Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DataSet<T>
where
    T: Ord + Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            set: BTreeSet::new(),
            on_item_added: Event::new(),
            on_item_deleted: Event::new(),
            on_cleared: Event::new(),
            guard: Box::new(|_| true),
        }
    }

    pub fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut set = Self::new();
        for item in iterable {
            set.set.insert(item);
        }
        set
    }

    pub fn clear(&mut self) {
        self.set.clear();
        self.on_cleared.trigger(Some(()));
    }

    pub fn add(&mut self, values: &[T]) {
        for item in values {
            if self.set.contains(item) {
                continue;
            }
            if !(self.guard)(item) {
                continue;
            }
            self.set.insert(item.clone());
            self.on_item_added.trigger(Some(item.clone()));
        }
    }

    pub fn delete(&mut self, value: &T) -> bool {
        let deleted = self.set.remove(value);
        if deleted {
            self.on_item_deleted.trigger(Some(()));
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
