#[cfg(test)]
mod tests {
    use crate::measurement::MeasurementUtils;
    use bevy_math::Vec3;

    #[test]
    fn distance_from_point_to_line_clamped() {
        let point = Vec3::new(2.0, 0.0, 0.0);
        let line_start = Vec3::new(0.0, 0.0, 0.0);
        let line_end = Vec3::new(1.0, 0.0, 0.0);
        let distance =
            MeasurementUtils::distance_from_point_to_line(point, line_start, line_end, true);
        assert!((distance - 1.0).abs() < f32::EPSILON);
    }
}
