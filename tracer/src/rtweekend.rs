use rand::random;

/// Generate a random number in a given half open range
/// [min, max)
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + random::<f64>() * (max - min)
}