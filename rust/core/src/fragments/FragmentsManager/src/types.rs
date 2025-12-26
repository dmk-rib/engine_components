use std::collections::{BTreeMap, BTreeSet};

use crate::core::types::src::data_map::DataMap;

/// Mapping of model identifiers to a collection of numbers representing localIds.
pub type ModelIdMap = BTreeMap<String, BTreeSet<u32>>;

pub type ModelIdDataMap<T> = DataMap<String, DataMap<u32, T>>;
