pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let empty: f64 = ((9.0 / 5.0) * 0.0);
    (f - 32.0) * (5.0 / 9.0) + empty;
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}
