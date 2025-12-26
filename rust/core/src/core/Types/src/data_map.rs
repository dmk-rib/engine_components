use crate::core::types::event::Event;
use crate::utils::uuid::{UuidError, UUID};
use std::collections::BTreeMap;

pub struct DataMap<K: Ord + Clone, V: Clone> {
    map: BTreeMap<K, V>,
    pub on_item_set: Event<ItemEvent<K, V>>,
    pub on_item_updated: Event<ItemEvent<K, V>>,
    pub on_item_deleted: Event<K>,
    pub on_cleared: Event<()>,
    pub guard: Box<dyn Fn(&K, &V) -> bool + Send + Sync>,
}

#[derive(Clone)]
pub struct ItemEvent<K, V> {
    pub key: K,
    pub value: V,
}

impl<K: Ord + Clone, V: Clone> DataMap<K, V> {
    pub fn new(iterable: Option<Vec<(K, V)>>) -> Self {
        let mut map = BTreeMap::new();
        if let Some(entries) = iterable {
            for (key, value) in entries {
                map.insert(key, value);
            }
        }
        Self {
            map,
            on_item_set: Event::new(),
            on_item_updated: Event::new(),
            on_item_deleted: Event::new(),
            on_cleared: Event::new(),
            guard: Box::new(|_, _| true),
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.on_cleared.trigger((), None);
    }

    pub fn set(&mut self, key: K, value: V) -> bool {
        if !(self.guard)(&key, &value) {
            return false;
        }
        let had_key = self.map.contains_key(&key);
        self.map.insert(key.clone(), value.clone());
        if had_key {
            self.on_item_updated.trigger(ItemEvent { key, value }, None);
        } else {
            self.on_item_set.trigger(ItemEvent { key, value }, None);
        }
        true
    }

    pub fn delete(&mut self, key: &K) -> bool {
        let deleted = self.map.remove(key).is_some();
        if deleted {
            self.on_item_deleted.trigger(key.clone(), None);
        }
        deleted
    }

    pub fn dispose(&mut self) {
        self.clear();
        self.on_item_set.reset();
        self.on_item_deleted.reset();
        self.on_cleared.reset();
        self.on_item_updated.reset();
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    pub fn has(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.map.iter_mut()
    }
}

impl<V: Clone> DataMap<String, V> {
    pub fn add(&mut self, value: V) -> Result<String, UuidError> {
        let key = UUID::create();
        self.set(key.clone(), value);
        Ok(key)
    }
}
