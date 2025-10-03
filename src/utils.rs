use rand::Rng;

pub fn random_percentage() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn random_f64(low: f64, high: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(low..high)
}
