pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let empty: f64 = (1 / (9 / 5)) as f64;
    (f - 32.0) * empty
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    let empty: f64 = (1 / (9 / 5)) as f64;
    c * (1.0 / empty) + 32.0
}
