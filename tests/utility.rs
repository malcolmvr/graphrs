#[allow(dead_code)]

/// rounds an &64 to a specified number of `decimal_places`
pub fn round(number: &f64, decimal_places: u32) -> f64 {
    let x = (10.0 as i32).pow(decimal_places) as f64;
    (number * x).round() / x
}
