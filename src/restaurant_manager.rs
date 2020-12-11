use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::Dish;
use crate::errors::RestaurantError;

/// The RestaurantState struct represents the shared Actix application state service that
/// will be used by the various application handlers to create, get or delete orders
/// We assume this restaurant only has 100 tables available.
pub struct RestaurantStateManager(Arc<RwLock<HashMap<usize, Vec<Dish>>>>);

/// The DishDto protect the internal representation of the Dish data struct,
/// making sure that the API client can only define the order for a new Dish with the "name"
/// parameter, while the "preparation" time value us still calculated automatically
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DishDto {
    pub name: String
}

impl RestaurantStateManager {
    pub fn instance() -> Self {
        RestaurantStateManager(Arc::new(RwLock::new(HashMap::new())))
    }

    pub fn new_order(&self, table_number: usize, dish: DishDto) -> Result<(), RestaurantError> {
        match table_number {
            1..=100 => {
                let mut dishes_for_table = self.0
                                               .try_write()
                                               .expect("could not acquire write lock");
                if let Some(dishes) = dishes_for_table.get_mut(&table_number) {
                    dishes.push(Dish::of(dish.name));
                    Ok(())
                } else {
                    dishes_for_table.insert(table_number, vec![Dish::of(dish.name)]);
                    Ok(())
                }
            }
            _ => Err(RestaurantError::TableNumberMismatchError)
        }
    }

    /**
    TODO: clarify the specification for "delete_order" operations: at the moment this will
    delete ALL the dishes with the same name from the order. Should it remove just one dish
    at time?
     */
    pub fn delete_order(&self, table_number: usize, dish_name: String) -> Result<(), RestaurantError> {
        match table_number {
            1..=100 => {
                if let Some(dishes) = self.0
                                          .try_write()
                                          .expect("could not acquire write lock")
                                          .get_mut(&table_number) {
                    Vec::retain(dishes, |dish| dish_name != *dish.name());
                }
                Ok(())
            }
            _ => Err(RestaurantError::TableNumberMismatchError)
        }
    }

    pub(crate) fn retrieve_orders(&self, table_number: usize) -> Result<Vec<Dish>, RestaurantError> {
        match table_number {
            1..=100 => Ok(
                self.0.try_read().expect("could not acquire read lock")
                    .get(&table_number)
                    .map(|dish| dish.clone())
                    .unwrap_or(vec![])
            ),
            _ => Err(RestaurantError::TableNumberMismatchError)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use parking_lot::RwLock;

    use crate::Dish;
    use crate::restaurant_manager::{DishDto, RestaurantStateManager};

    #[test]
    fn test_orders_are_no_accepted_if_table_number_out_of_bounds() {
        let orders = Arc::new(RwLock::new(HashMap::<usize, Vec<Dish>>::new()));
        let state_manager = RestaurantStateManager(orders.clone());

        let dishes_for_table = state_manager.retrieve_orders(82726);
        assert!(dishes_for_table.is_err());
        let dishes_for_table = state_manager.retrieve_orders(0);
        assert!(dishes_for_table.is_err());
    }

    #[test]
    fn test_dishes_can_be_deleted_from_orders() {
        let orders = Arc::new(RwLock::new(HashMap::<usize, Vec<Dish>>::new()));
        let state_manager = RestaurantStateManager(orders.clone());

        state_manager.0.try_write()
                     .expect("could not acquire write lock")
                     .insert(1, vec![Dish::of("お好み焼き".to_string())]);
        state_manager
            .delete_order(1, "お好み焼き".to_string())
            .expect("could not delete the selected dish.");

        assert!(state_manager.0.try_read()
                             .expect("could not acquire read lock")
                             .get(&1)
                             .expect("could access orders map")
                             .is_empty());
    }

    #[test]
    fn test_ordered_dishes_can_be_retrieved_from_orders_list() {
        let orders = Arc::new(RwLock::new(HashMap::<usize, Vec<Dish>>::new()));
        let state_manager = RestaurantStateManager(orders.clone());

        state_manager.0.try_write()
                     .expect("could not acquire write lock")
                     .insert(7, vec![Dish::of(String::from("some_kind_of_food"))]);

        let dishes = state_manager.retrieve_orders(7);
        assert!(dishes.is_ok());
        let more_dishes = dishes.unwrap();
        assert_eq!(1, more_dishes.len());
        assert_eq!(String::from("some_kind_of_food"), more_dishes.get(0).unwrap().name);
    }

    #[test]
    fn test_dishes_with_same_name_are_saved_in_the_order() {
        let orders = Arc::new(RwLock::new(HashMap::<usize, Vec<Dish>>::new()));
        let state_manager = RestaurantStateManager(orders.clone());

        // make a lot of lasagna...
        for _ in 0..5 {
            let multiple_lasagna =
                state_manager.new_order(42, DishDto { name: String::from("lasagna") });
            assert!(multiple_lasagna.is_ok());
        }
        let orders_map = state_manager.0.try_read()
                                      .expect("could not acquire read lock");
        // they should have a lot of lasagna on table 42 right now :)
        let mut dishes = orders_map.get(&42);
        assert!(dishes.is_some());
        assert_eq!(5, dishes.as_mut().unwrap().len());
    }
}