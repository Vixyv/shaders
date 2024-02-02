// A *much* faster floor algorithm (about 6x)
pub fn fast_floor(x: f64) -> i64 {
    if x > 0.0 { x as i64 } else { 1 - x as i64 }
}
