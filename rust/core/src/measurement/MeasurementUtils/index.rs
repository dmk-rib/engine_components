use crate::core::components::{
    ComponentFactory, ComponentInstance, ComponentsError, ComponentsHandle,
};
use crate::core::types::src::component::Component;
use bevy_math::Vec3;
use std::any::Any;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct MeasureEdge {
    pub distance: f32,
    pub points: Vec<Vec3>,
}

pub struct MeasurementUtils {
    pub component: Component,
}

impl MeasurementUtils {
    pub const UUID: &'static str = "267ca032-672f-4cb0-afa9-d24e904f39d6";

    pub fn new(
        components: ComponentsHandle,
    ) -> Result<Arc<Mutex<dyn ComponentInstance>>, ComponentsError> {
        let instance = Arc::new(Mutex::new(Self {
            component: Component::new(components.clone()),
        }));
        components
            .lock()
            .expect("components lock poisoned")
            .add(Self::UUID, instance.clone())?;
        Ok(instance)
    }

    pub fn distance_from_point_to_line(
        point: Vec3,
        line_start: Vec3,
        line_end: Vec3,
        clamp: bool,
    ) -> f32 {
        let line = line_end - line_start;
        let line_length_squared = line.length_squared();
        if line_length_squared == 0.0 {
            return point.distance(line_start);
        }

        let mut t = (point - line_start).dot(line) / line_length_squared;
        if clamp {
            t = t.clamp(0.0, 1.0);
        }
        let projection = line_start + line * t;
        projection.distance(point)
    }
}

impl ComponentInstance for MeasurementUtils {
    fn enabled(&self) -> bool {
        self.component.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.component.enabled = enabled;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ComponentFactory for MeasurementUtils {
    const UUID: &'static str = MeasurementUtils::UUID;

    fn new(components: ComponentsHandle) -> Self {
        Self {
            component: Component::new(components),
        }
    }
}
