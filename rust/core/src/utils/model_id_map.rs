use crate::fragments::fragments_manager::types::ModelIdMap;
use std::collections::{BTreeMap, BTreeSet};

pub struct ModelIdMapUtils;

impl ModelIdMapUtils {
    pub fn join(maps: &[ModelIdMap]) -> ModelIdMap {
        let mut result: ModelIdMap = BTreeMap::new();

        for map in maps {
            for (model_id, ids) in map {
                let entry = result.entry(model_id.clone()).or_insert_with(BTreeSet::new);
                entry.extend(ids.iter().copied());
            }
        }

        result
    }

    pub fn intersect(maps: &[ModelIdMap]) -> ModelIdMap {
        if maps.is_empty() {
            return BTreeMap::new();
        }

        let mut result = Self::clone_map(&maps[0]);

        for map in &maps[1..] {
            let mut next: ModelIdMap = BTreeMap::new();
            for (model_id, ids) in &result {
                if let Some(other_ids) = map.get(model_id) {
                    let intersection: BTreeSet<u32> = ids
                        .iter()
                        .copied()
                        .filter(|id| other_ids.contains(id))
                        .collect();
                    if !intersection.is_empty() {
                        next.insert(model_id.clone(), intersection);
                    }
                }
            }
            result = next;
        }

        result
    }

    pub fn clone_map(source: &ModelIdMap) -> ModelIdMap {
        source
            .iter()
            .map(|(model_id, ids)| (model_id.clone(), ids.clone()))
            .collect()
    }

    /// Rust-friendly mapping: returns the resulting map when clone is requested.
    pub fn remove(target: &mut ModelIdMap, source: &ModelIdMap, clone: bool) -> ModelIdMap {
        let mut output = if clone {
            Self::clone_map(target)
        } else {
            std::mem::take(target)
        };

        for (model_id, ids) in source {
            if let Some(target_ids) = output.get_mut(model_id) {
                for id in ids {
                    target_ids.remove(id);
                }
                if target_ids.is_empty() {
                    output.remove(model_id);
                }
            }
        }

        if !clone {
            *target = output.clone();
        }

        output
    }

    /// Rust-friendly mapping: returns the resulting map when clone is requested.
    pub fn add(target: &mut ModelIdMap, source: &ModelIdMap, clone: bool) -> ModelIdMap {
        let mut output = if clone {
            Self::clone_map(target)
        } else {
            std::mem::take(target)
        };

        for (model_id, ids) in source {
            let entry = output.entry(model_id.clone()).or_insert_with(BTreeSet::new);
            entry.extend(ids.iter().copied());
        }

        if !clone {
            *target = output.clone();
        }

        output
    }

    pub fn append(target: &mut ModelIdMap, model_id: &str, local_ids: &[u32]) {
        let entry = target
            .entry(model_id.to_string())
            .or_insert_with(BTreeSet::new);
        entry.extend(local_ids.iter().copied());
    }

    pub fn is_equal(a: &ModelIdMap, b: &ModelIdMap) -> bool {
        if a.len() != b.len() {
            return false;
        }

        for (model_id, ids) in a {
            match b.get(model_id) {
                Some(other_ids) if ids == other_ids => {}
                _ => return false,
            }
        }

        true
    }

    pub fn is_empty(map: &ModelIdMap) -> bool {
        map.values().all(|set| set.is_empty())
    }

    pub fn to_raw(map: &ModelIdMap) -> BTreeMap<String, Vec<u32>> {
        map.iter()
            .map(|(model_id, ids)| (model_id.clone(), ids.iter().copied().collect()))
            .collect()
    }

    pub fn from_raw(raw: &BTreeMap<String, Vec<u32>>) -> ModelIdMap {
        raw.iter()
            .map(|(model_id, ids)| (model_id.clone(), ids.iter().copied().collect()))
            .collect()
    }
}
