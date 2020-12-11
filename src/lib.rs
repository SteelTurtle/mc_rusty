#[macro_use]
extern crate serde;
extern crate serde_json;

use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod errors;
pub mod handlers;
pub mod restaurant_manager;

// for the sake of this scenario a dish is considered "ready and delivered"
// when the preparation time reaches values less or equal to zero
// Also, "seconds" == "minutes" in this scenario
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Dish {
    name: String,
    preparation: i32,
}

impl Dish {
    pub fn of(name: String) -> Self {
        Self {
            name,
            preparation: rand::thread_rng().gen_range(5, 15),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooking_time_between_5_and_15() {
        let min = 5;
        let max = 15;
        let dish = Dish::of(String::from("pizza"));
        let preparation_time = dish.preparation;
        assert!(preparation_time <= max && preparation_time >= min);
    }
}

