// use json::JsonValue;
pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_calories = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;

    for food in foods {
        total_calories += food.calories.1.trim_end_matches("kcal").parse::<f64>().unwrap_or(0.0) * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
    }

    json::object! {
        "total_calories" => format!("{:.2} kcal", total_calories),
        "total_proteins" => format!("{:.2} g", total_proteins),
        "total_fats" => format!("{:.2} g", total_fats),
        "total_carbs" => format!("{:.2} g", total_carbs),
    }
}


