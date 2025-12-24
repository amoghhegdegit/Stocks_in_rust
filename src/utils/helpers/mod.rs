use uuid::Uuid;

pub fn generate_uuid_v4() -> Uuid {
    Uuid::new_v4()
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min { min } else if value > max { max } else { value }
}

pub fn round(value: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor
}