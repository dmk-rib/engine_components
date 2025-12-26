use std::collections::{BTreeMap, BTreeSet};

/// Mapping of model identifiers to a collection of numbers representing localIds.
pub type ModelIdMap = BTreeMap<String, BTreeSet<u32>>;

pub type ModelIdDataMap<T> = BTreeMap<String, BTreeMap<u32, T>>;
