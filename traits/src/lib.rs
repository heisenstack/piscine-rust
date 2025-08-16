use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub name: String,       
    pub strength: f64,  
    pub score: i32,        
    pub money: i32,         
    pub weapons: Vec<String>, 
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64, 
    pub fat_content: f64, 
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        4.0 * self.weight_in_kg
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat_kg = self.weight_in_kg * self.fat_content;      
        let protein_kg = self.weight_in_kg * (1.0 - self.fat_content); 

        (fat_kg * 9.0) + (protein_kg * 4.0)
    }
}

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
    pub fn eat<T: Food>(&mut self, food: T) {
        self.strength += food.gives();
    }
}
