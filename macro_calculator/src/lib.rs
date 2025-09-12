pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;
    for f in foods {
        let p = f.nbr_of_portions;
        cals += num(&f.calories.1) * p;
        carbs += f.carbs * p;
        proteins += f.proteins * p;
        fats += f.fats * p;
    }
    json::object! { cals:round(cals), carbs:round(carbs), proteins:round(proteins), fats:round(fats) }
}

pub fn round(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}
pub fn num(s: &str) -> f64 {
    let mut res = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() || c == '.' || c == '-' {
            res.push(c);
        }
    }
    res.parse().unwrap_or(0.0)
}
