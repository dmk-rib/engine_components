use std::collections::BTreeMap;

use crate::core::types::src::event::Event;
use crate::utils::uuid::UUID;

pub struct DataMap<K, V>
where
    K: Ord + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
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

impl<K, V> Default for DataMap<K, V>
where
    K: Ord + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> DataMap<K, V>
where
    K: Ord + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            on_item_set: Event::new(),
            on_item_updated: Event::new(),
            on_item_deleted: Event::new(),
            on_cleared: Event::new(),
            guard: Box::new(|_, _| true),
        }
    }

    pub fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut map = Self::new();
        for (key, value) in iterable {
            map.map.insert(key, value);
        }
        map
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.on_cleared.trigger(Some(()));
    }

    pub fn set(&mut self, key: K, value: V) {
        let trigger_update = self.map.contains_key(&key);
        if !(self.guard)(&key, &value) {
            return;
        }
        self.map.insert(key.clone(), value.clone());
        if trigger_update {
            self.on_item_updated.trigger(Some(ItemEvent { key, value }));
        } else {
            self.on_item_set.trigger(Some(ItemEvent { key, value }));
        }
    }

    pub fn add(&mut self, value: V) -> String
    where
        K: From<String>,
    {
        let key = UUID::create();
        self.set(K::from(key.clone()), value);
        key
    }

    pub fn delete(&mut self, key: &K) -> bool {
        let deleted = self.map.remove(key).is_some();
        if deleted {
            self.on_item_deleted.trigger(Some(key.clone()));
        }
        deleted
    }

    pub fn dispose(&mut self) {
        self.clear();
        self.on_item_set.reset();
        self.on_item_deleted.reset();
        self.on_cleared.reset();
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}
