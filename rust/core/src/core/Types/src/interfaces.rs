use crate::core::types::event::Event;
use crate::core::types::event_manager::EventManager;
use bevy_color::Color;
use bevy_math::{Vec2, Vec3};

pub trait Disposable {
    fn dispose(&mut self);
    fn on_disposed(&self) -> &Event<()>;
}

pub trait Hideable {
    fn visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
}

pub trait Resizeable {
    fn resize(&mut self, size: Option<Vec2>);
    fn on_resize(&self) -> &Event<Vec2>;
    fn size(&self) -> Vec2;
}

pub trait Updateable {
    fn on_after_update(&self) -> &Event<()>;
    fn on_before_update(&self) -> &Event<()>;
    fn update(&mut self, delta: Option<f32>);
}

#[derive(Debug, Clone, Copy)]
pub struct Progress {
    pub current: u64,
    pub total: u64,
}

pub trait Createable<T> {
    fn create(&mut self, data: T);
    fn end_creation(&mut self, _data: T) {}
    fn cancel_creation(&mut self, _data: T) {}
    fn delete(&mut self, data: T);
}

pub trait Configurable<T, U> {
    fn is_setup(&self) -> bool;
    fn setup(&mut self, config: Option<U>);
    fn on_setup(&self) -> &Event<()>;
    fn config(&self) -> &T;
}

#[derive(Debug, Default, Clone)]
pub struct CameraControls;

pub trait CameraControllable {
    fn controls(&self) -> &CameraControls;
}

pub trait Eventable {
    fn event_manager(&self) -> &EventManager;
}

#[derive(Debug, Default, Clone)]
pub struct HtmlElement;

pub type UiFactory = Box<dyn Fn() -> HtmlElement + Send + Sync>;

pub trait WithUi {
    fn ui(&self) -> &std::collections::BTreeMap<String, UiFactory>;
}

#[derive(Debug, Clone)]
pub struct SerializationResult<
    D = std::collections::BTreeMap<String, String>,
    S = std::collections::BTreeMap<String, String>,
> {
    pub data: Option<Vec<D>>,
    pub settings: Option<S>,
}

pub trait Serializable<
    D = std::collections::BTreeMap<String, String>,
    S = std::collections::BTreeMap<String, String>,
>
{
    fn import(&mut self, result: SerializationResult<D, S>);
    fn export(&self) -> SerializationResult<D, S>;
}

#[derive(Debug, Clone, Copy)]
pub struct BooleanSettingsControl {
    pub value: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorSettingsControl {
    pub value: Color,
}

#[derive(Debug, Clone)]
pub struct TextSettingsControl {
    pub value: String,
}

#[derive(Debug, Clone, Copy)]
pub struct NumberSettingControl {
    pub interpolable: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct SelectSettingControl {
    pub multiple: bool,
    pub options: std::collections::BTreeSet<String>,
    pub value: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3SettingControl {
    pub value: Vec3,
}

#[derive(Debug, Clone)]
pub struct TextSetSettingControl {
    pub value: std::collections::BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct NoControl {
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum ControlEntry {
    Boolean(BooleanSettingsControl),
    Color(ColorSettingsControl),
    Text(TextSettingsControl),
    Number(NumberSettingControl),
    Select(SelectSettingControl),
    Vector3(Vector3SettingControl),
    TextSet(TextSetSettingControl),
    None(NoControl),
}

pub type ControlsSchema = std::collections::BTreeMap<String, ControlsSchemaEntry>;

#[derive(Debug, Clone)]
pub enum ControlsSchemaEntry {
    Entry(ControlEntry),
    Schema(ControlsSchema),
}

pub struct ControlsUtils;

impl ControlsUtils {
    pub fn is_entry(item: &ControlsSchemaEntry) -> bool {
        matches!(item, ControlsSchemaEntry::Entry(_))
    }

    pub fn copy_schema(schema: &ControlsSchema, copy: &mut ControlsSchema) {
        for (name, entry) in schema {
            match entry {
                ControlsSchemaEntry::Entry(control_entry) => {
                    copy.insert(
                        name.clone(),
                        ControlsSchemaEntry::Entry(Self::copy_entry(control_entry)),
                    );
                }
                ControlsSchemaEntry::Schema(nested) => {
                    let mut nested_copy = ControlsSchema::new();
                    Self::copy_schema(nested, &mut nested_copy);
                    copy.insert(name.clone(), ControlsSchemaEntry::Schema(nested_copy));
                }
            }
        }
    }

    pub fn copy_entry(control_entry: &ControlEntry) -> ControlEntry {
        match control_entry {
            ControlEntry::Boolean(entry) => ControlEntry::Boolean(*entry),
            ControlEntry::Color(entry) => ControlEntry::Color(*entry),
            ControlEntry::Text(entry) => ControlEntry::Text(entry.clone()),
            ControlEntry::Number(entry) => ControlEntry::Number(*entry),
            ControlEntry::Select(entry) => ControlEntry::Select(entry.clone()),
            ControlEntry::Vector3(entry) => ControlEntry::Vector3(*entry),
            ControlEntry::TextSet(entry) => ControlEntry::TextSet(entry.clone()),
            ControlEntry::None(entry) => ControlEntry::None(entry.clone()),
        }
    }
}
