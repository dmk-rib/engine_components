use crate::core::components::{ComponentFactory, ComponentInstance, ComponentsHandle};
use crate::core::types::component::Component;
use crate::fragments::fragments_manager::types::ModelIdMap;
use crate::fragments::fragments_manager::FragmentsManager;
use bevy_math::Vec3;
use std::any::Any;
use std::f32;

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

    pub fn round(vector: &mut Vec3) {
        let factor = 1000.0;
        vector.x = (vector.x * factor).trunc() / factor;
        vector.y = (vector.y * factor).trunc() / factor;
        vector.z = (vector.z * factor).trunc() / factor;
    }

    pub async fn get_volume_from_fragments(&self, model_id_map: &ModelIdMap) -> f64 {
        self.get_items_volume(model_id_map).await
    }

    pub async fn get_items_volume(&self, model_id_map: &ModelIdMap) -> f64 {
        let mut volume = 0.0;
        let components = self.component.base.components.clone();
        let fragments = {
            let components = components.lock().expect("components lock");
            components.get_component::<FragmentsManager>()
        };

        let Some(fragments) = fragments else {
            return volume;
        };

        let mut fragments = fragments.lock().expect("fragments lock");
        let Some(fragments) = fragments.as_any_mut().downcast_mut::<FragmentsManager>() else {
            return volume;
        };
        for (model_id, local_ids) in model_id_map {
            if let Some(model) = fragments.list.get(model_id) {
                let ids: Vec<u32> = local_ids.iter().copied().collect();
                volume += model.get_items_volume(&ids).await;
            }
        }
        volume
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

impl ComponentInstance for MeasurementUtils {
    fn enabled(&self) -> bool {
        self.component.enabled()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.component.set_enabled(enabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod test;
