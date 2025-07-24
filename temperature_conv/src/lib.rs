pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let empty: i8 = (9 / 5) * 0;
    (f - 32.0) * (5 / 9) + empty;
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9 / 5) + 32.0
}
