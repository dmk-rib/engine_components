use std::collections::{BTreeMap, BTreeSet};

use bevy_color::Color;
use bevy_math::Vec3;

use crate::core::types::src::interfaces::JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub enum ControlEntry {
    Boolean {
        value: bool,
    },
    Color {
        value: Color,
    },
    Text {
        value: String,
    },
    Number {
        interpolable: bool,
        min: Option<f32>,
        max: Option<f32>,
        value: f32,
    },
    Select {
        multiple: bool,
        options: BTreeSet<String>,
        value: String,
    },
    Vector3 {
        value: Vec3,
    },
    TextSet {
        value: BTreeSet<String>,
    },
    None {
        value: JsonValue,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlEntryOrSchema {
    Entry(ControlEntry),
    Schema(ControlsSchema),
}

pub type ControlsSchema = BTreeMap<String, ControlEntryOrSchema>;

pub struct ControlsUtils;

impl ControlsUtils {
    pub fn is_entry(item: &ControlEntryOrSchema) -> bool {
        matches!(item, ControlEntryOrSchema::Entry(_))
    }

    pub fn copy_schema(schema: &ControlsSchema, copy: &mut ControlsSchema) {
        for (name, entry) in schema {
            match entry {
                ControlEntryOrSchema::Entry(entry) => {
                    copy.insert(name.clone(), ControlEntryOrSchema::Entry(entry.clone()));
                }
                ControlEntryOrSchema::Schema(child) => {
                    let mut child_copy = ControlsSchema::new();
                    Self::copy_schema(child, &mut child_copy);
                    copy.insert(name.clone(), ControlEntryOrSchema::Schema(child_copy));
                }
            }
        }
    }

    pub fn copy_entry(control_entry: &ControlEntry) -> ControlEntry {
        control_entry.clone()
    }
}
