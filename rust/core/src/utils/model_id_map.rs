use std::collections::{BTreeMap, BTreeSet};

use crate::fragments::ModelIdMap;

pub struct ModelIdMapUtils;

impl ModelIdMapUtils {
    pub fn join(maps: &[ModelIdMap]) -> ModelIdMap {
        let mut result: ModelIdMap = BTreeMap::new();
        for map in maps {
            for (model_id, ids) in map {
                let entry = result.entry(model_id.clone()).or_default();
                entry.extend(ids.iter().copied());
            }
        }
        result
    }

    pub fn intersect(maps: &[ModelIdMap]) -> ModelIdMap {
        if maps.is_empty() {
            return BTreeMap::new();
        }

        let mut result = maps[0].clone();
        for current in &maps[1..] {
            let mut new_result: ModelIdMap = BTreeMap::new();
            for (model_id, ids) in &result {
                if let Some(current_ids) = current.get(model_id) {
                    let intersection: BTreeSet<u32> =
                        ids.intersection(current_ids).copied().collect();
                    if !intersection.is_empty() {
                        new_result.insert(model_id.clone(), intersection);
                    }
                }
            }
            result = new_result;
        }

        result
    }

    pub fn clone(source: &ModelIdMap) -> ModelIdMap {
        source.clone()
    }

    /// Rust-friendly mapping of the optional clone flag.
    /// Returns the resulting map and mutates `target` only when `clone` is false.
    pub fn remove(target: &mut ModelIdMap, source: &ModelIdMap, clone: bool) -> ModelIdMap {
        let mut working = if clone {
            target.clone()
        } else {
            target.clone()
        };
        for (model_id, ids) in source {
            if let Some(target_ids) = working.get_mut(model_id) {
                for id in ids {
                    target_ids.remove(id);
                }
                if target_ids.is_empty() {
                    working.remove(model_id);
                }
            }
        }
        if !clone {
            *target = working.clone();
        }
        working
    }

    /// Rust-friendly mapping of the optional clone flag.
    /// Returns the resulting map and mutates `target` only when `clone` is false.
    pub fn add(target: &mut ModelIdMap, source: &ModelIdMap, clone: bool) -> ModelIdMap {
        let mut working = if clone {
            target.clone()
        } else {
            target.clone()
        };
        for (model_id, ids) in source {
            let entry = working.entry(model_id.clone()).or_default();
            entry.extend(ids.iter().copied());
        }
        if !clone {
            *target = working.clone();
        }
        working
    }

    pub fn append(target: &mut ModelIdMap, model_id: &str, local_ids: &[u32]) {
        let entry = target
            .entry(model_id.to_string())
            .or_insert_with(BTreeSet::new);
        for local_id in local_ids {
            entry.insert(*local_id);
        }
    }

    pub fn is_equal(a: &ModelIdMap, b: &ModelIdMap) -> bool {
        a == b
    }

    pub fn is_empty(map: &ModelIdMap) -> bool {
        map.values().all(|set| set.is_empty())
    }

    pub fn to_raw(map: &ModelIdMap) -> BTreeMap<String, Vec<u32>> {
        let mut result = BTreeMap::new();
        for (model_id, ids) in map {
            result.insert(model_id.clone(), ids.iter().copied().collect());
        }
        result
    }

    pub fn from_raw(raw: &BTreeMap<String, Vec<u32>>) -> ModelIdMap {
        let mut result: ModelIdMap = BTreeMap::new();
        for (model_id, ids) in raw {
            result.insert(model_id.clone(), ids.iter().copied().collect());
        }
        result
    }
}
