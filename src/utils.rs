use rand::Rng;

/// Returns a value between 0.0 and 1.0
pub fn random_percentage() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn random_f64(low: f64, high: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(low..high)
}

pub fn random_u64(low: u64, high: u64) -> u64 {
    let mut rng = rand::rng();
    rng.random_range(low..high + 1)
}
