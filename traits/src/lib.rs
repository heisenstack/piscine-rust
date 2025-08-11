use std::fmt;

/// Represents a player in the game
#[derive(Debug)]
pub struct Player {
    pub name: String,       // Player's name
    pub strength: f64,      // Player's current strength
    pub score: i32,         // Points earned
    pub money: i32,         // Currency the player has
    pub weapons: Vec<String>, // List of weapon names
}

/// Represents a fruit with a certain weight
pub struct Fruit {
    pub weight_in_kg: f64, // Weight in kilograms
}

/// Represents a piece of meat with a certain weight and fat percentage
pub struct Meat {
    pub weight_in_kg: f64, // Weight in kilograms
    pub fat_content: f64,  // Percentage of fat (0.0 to 1.0)
}

/// Trait that all foods must implement
/// It returns how much strength the food gives
pub trait Food {
    fn gives(&self) -> f64;
}

/// Implementation of `Food` for Fruit
/// Each kg of fruit gives 4 strength points
impl Food for Fruit {
    fn gives(&self) -> f64 {
        4.0 * self.weight_in_kg
    }
}

/// Implementation of `Food` for Meat
/// Strength gain depends on fat and protein content
impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat_kg = self.weight_in_kg * self.fat_content;       // Fat weight in kg
        let protein_kg = self.weight_in_kg * (1.0 - self.fat_content); // Protein weight in kg

        // Fat gives 9 strength per kg, protein gives 4 strength per kg
        (fat_kg * 9.0) + (protein_kg * 4.0)
    }
}

/// Custom display for Player
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(
            f,
            "Strength: {}, Score: {}, Money: {}",
            self.strength as i64,
            self.score,
            self.money
        )?;
        write!(f, "Weapons: {:?}", self.weapons)
    }
}

impl Player {
    /// Allows the player to eat any type of food that implements `Food`
    pub fn eat<T: Food>(&mut self, food: T) {
        // Increase strength based on what the food gives
        self.strength += food.gives();
    }
}
