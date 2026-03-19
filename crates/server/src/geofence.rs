// =============================================================================
// Geofencing utilities — haversine distance + duration anomaly detection
// =============================================================================

/// Calculate the haversine distance between two points in meters.
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6_371_000.0; // Earth radius in meters

    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();

    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    R * c
}

/// Calculate the ratio of actual to scheduled duration.
/// Returns the ratio (e.g., 0.5 means actual was 50% of scheduled).
pub fn duration_ratio(
    scheduled_minutes: f64,
    actual_minutes: f64,
) -> f64 {
    if scheduled_minutes <= 0.0 {
        return 1.0;
    }
    actual_minutes / scheduled_minutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_same_point() {
        let d = haversine_distance(37.5665, 126.978, 37.5665, 126.978);
        assert!(d < 1.0); // Should be ~0 meters
    }

    #[test]
    fn test_haversine_known_distance() {
        // Seoul (37.5665, 126.978) to Busan (35.1796, 129.0756) ~ 325km
        let d = haversine_distance(37.5665, 126.978, 35.1796, 129.0756);
        assert!((d - 325_000.0).abs() < 10_000.0);
    }

    #[test]
    fn test_duration_ratio() {
        assert!((duration_ratio(60.0, 30.0) - 0.5).abs() < f64::EPSILON);
        assert!((duration_ratio(60.0, 60.0) - 1.0).abs() < f64::EPSILON);
    }
}
