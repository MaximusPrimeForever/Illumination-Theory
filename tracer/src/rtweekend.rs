use rand::random;

/// Generate a random number in a given half open range
/// [min, max)
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + random::<f64>() * (max - min)
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min { return min; }
    if value > max { return max; }
    value
}
