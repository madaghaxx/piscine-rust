use json::object;
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
    let mut fats = 0.0;
    let mut proteins = 0.0;

    for food in foods {
        let calories: f64 = food.calories.1[0..food.calories.1.len() - 4].parse().unwrap();
        cals += calories * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
    }

    let format_number = |n: f64| -> f64 {
        let rounded = (n * 100.0).round() / 100.0;
        if rounded.fract() == 0.0 {
            rounded
        } else if (rounded * 10.0).fract() == 0.0 {
            (rounded * 10.0).round() / 10.0
        } else {
            rounded
        }
    };

    object! {
        cals: format_number(cals),
        carbs: format_number(carbs),
        proteins: format_number(proteins),
        fats: format_number(fats)
    }
}
