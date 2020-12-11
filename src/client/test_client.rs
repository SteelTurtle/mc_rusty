#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate ureq;

use std::sync::{Arc, Mutex};
use std::thread;

use rand::{Rng, RngCore, thread_rng};

fn main() {
    /*
    TODO:  Ideally, the test client could be implemented as an independent command line tool in
     which we could specify on demand the server host, number of threads per requests type, etc.
     */
    start_test_client(300);
}

fn start_test_client(requests: i32) {
    let dish_names = Arc::new(Mutex::new(vec![
        "pizza",
        "豚カツ",
        "spaghetti",
        "steak",
        "mixed salad",
        "grilled fish",
        "ラーメン",
        "麻婆豆腐"
    ]));

    let mut request_handlers = vec![];
    for _ in 0..requests {
        let choice = thread_rng().next_u32() % (dish_names.lock().unwrap().len()) as u32;
        let dishes_clone = dish_names.clone();
        request_handlers.push(thread::spawn(move || {
            for _ in 0..5 {
                let table_number = thread_rng().gen_range(1, 100);
                test_client_config::send_new_order(table_number,
                                                   dishes_clone.lock().unwrap()[choice as usize]);
            }
        }))
    }

    for _ in 0..requests {
        request_handlers.push(thread::spawn(move || {
            for _ in 0..5 {
                let table_number = thread_rng().gen_range(1, 100);
                let _ = test_client_config::get_orders(table_number);
            }
        }))
    }

    for _ in 0..requests {
        let choice = thread_rng().next_u32() % (dish_names.lock().unwrap().len()) as u32;
        let dishes_clone = dish_names.clone();
        request_handlers.push(thread::spawn(move || {
            for _ in 0..5 {
                let table_number = thread_rng().gen_range(1, 100);
                test_client_config::delete_order(table_number,
                                                 dishes_clone.lock().unwrap()[choice as usize]);
            }
        }))
    }

    // wait for all threads completion
    for handle in request_handlers {
        handle.join().expect("error while waiting thread completion");
    }
}

pub(crate) mod test_client_config {
    use std::time::Duration;

    use ureq::{delete, get, post};

    use mc_rusty::Dish;
    use mc_rusty::errors::RestaurantError;

    #[derive(Serialize, Deserialize)]
    struct TestDishDto { name: String }

    pub(crate) fn send_new_order(table_number: usize, dish_name: &str) {
        let url = format!("http://127.0.0.1:9090/api/orders/{}", table_number);
        let response = post(url.as_str())
            .send_json(json!(TestDishDto{name: String::from(dish_name)}));
        assert!(response.ok());
    }

    pub(crate) fn get_orders(table_number: usize) -> Result<Vec<Dish>, RestaurantError> {
        let url = format!("http://127.0.0.1:9090/api/orders/{}", table_number);
        let response = get(url.as_str())
            .timeout_connect(1000)
            .timeout(Duration::from_secs(7))
            .call();
        assert!(response.ok());
        response.into_json_deserialize().unwrap()
    }

    pub(crate) fn delete_order(table_number: usize, dish_name: &str) {
        let url = format!("http://127.0.0.1:9090/api/orders/{}/{}", table_number, dish_name);
        let response = delete(url.as_str()).call();
        assert!(response.ok());
    }
}