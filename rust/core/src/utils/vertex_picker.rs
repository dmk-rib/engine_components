use crate::core::components::{ComponentInstance, ComponentsHandle};
use crate::core::raycasters::Raycasters;
use crate::core::types::base_renderer::Plane;
use crate::core::types::event::Event;
use crate::core::types::interfaces::{Disposable, HtmlElement};
use crate::core::types::world::WorldHandle;
use bevy_math::Vec3;

#[derive(Clone)]
pub struct VertexPickerConfig {
    pub show_only_vertex: bool,
    pub snap_distance: f32,
    pub preview_element: Option<HtmlElement>,
}

pub struct VertexPicker {
    pub on_disposed: Event<()>,
    pub on_vertex_found: Event<Vec3>,
    pub on_vertex_lost: Event<Vec3>,
    pub on_enabled: Event<bool>,
    pub working_plane: Option<Plane>,
    pub components: ComponentsHandle,
    picked_point: Option<Vec3>,
    config: VertexPickerConfig,
    enabled: bool,
}

impl VertexPicker {
    pub fn new(components: ComponentsHandle, config: Option<VertexPickerConfig>) -> Self {
        let mut picker = Self {
            on_disposed: Event::new(),
            on_vertex_found: Event::new(),
            on_vertex_lost: Event::new(),
            on_enabled: Event::new(),
            working_plane: None,
            components,
            picked_point: None,
            config: VertexPickerConfig {
                snap_distance: 0.25,
                show_only_vertex: false,
                preview_element: None,
            },
            enabled: false,
        };
        if let Some(config) = config {
            picker.set_config(config);
        }
        picker
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
        if !value {
            self.picked_point = None;
        }
        self.on_enabled.trigger(value, None);
    }

    pub fn config(&self) -> &VertexPickerConfig {
        &self.config
    }

    pub fn set_config(&mut self, value: VertexPickerConfig) {
        self.config = VertexPickerConfig {
            snap_distance: value.snap_distance,
            show_only_vertex: value.show_only_vertex,
            preview_element: value.preview_element,
        };
    }

    pub async fn get(&mut self, world: &WorldHandle) -> Option<Vec3> {
        if !self.enabled {
            return self.picked_point;
        }

        let raycasters = {
            let components = self.components.lock().expect("components lock");
            components.get_component::<Raycasters>()
        };

        let Some(raycasters) = raycasters else {
            return self.picked_point;
        };

        let mut raycasters = raycasters.lock().expect("raycasters lock");
        let Some(raycasters) = raycasters.as_any_mut().downcast_mut::<Raycasters>() else {
            return self.picked_point;
        };
        let caster = raycasters.get(world.clone());
        let intersects = caster.lock().expect("caster lock").cast_ray(None);
        let Some(intersects) = intersects else {
            if let Some(point) = self.picked_point.take() {
                self.on_vertex_lost.trigger(point, None);
            }
            return self.picked_point;
        };

        let point = intersects.point;

        if self
            .picked_point
            .as_ref()
            .map_or(true, |existing| *existing != point)
        {
            self.picked_point = Some(point);
            self.on_vertex_found.trigger(point, None);
        }

        self.picked_point
    }
}

impl Disposable for VertexPicker {
    fn dispose(&mut self) {
        self.on_vertex_found.reset();
        self.on_vertex_lost.reset();
        self.on_disposed.trigger((), None);
        self.on_disposed.reset();
    }

    fn on_disposed(&self) -> &Event<()> {
        &self.on_disposed
    }
}

impl ComponentInstance for VertexPicker {
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.set_enabled(enabled);
    }

    fn is_disposable(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
