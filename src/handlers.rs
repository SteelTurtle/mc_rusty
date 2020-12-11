use actix_web::{HttpResponse, Responder, web};

use crate::errors::RestaurantError;
use crate::restaurant_manager::{DishDto, RestaurantStateManager};

pub async fn add_order(table_number: web::Path<usize>,
                       dish: web::Json<DishDto>,
                       state_manager: web::Data<RestaurantStateManager>)
                       -> Result<HttpResponse, RestaurantError> {
    match state_manager.new_order(table_number.into_inner(),
                                  DishDto { name: dish.into_inner().name }) {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        _ => Err(RestaurantError::TableNumberMismatchError)
    }
}

/// As it is, the handler will return a json map called "Ok[]", wrapping
/// all the dishes data. If the desired result is just a simple json array not wrapped into a map
/// then the return type can be changed into a "Result<HttpResponse>", while the function should
/// return a "Ok(HttpResponse::Ok().json(state_manager.retrieve_orders(table_number.into_inner())
/// .unwrap()))"
pub async fn get_orders(table_number: web::Path<usize>,
                        state_manager: web::Data<RestaurantStateManager>)
                        -> impl Responder {
    web::Json(state_manager.retrieve_orders(table_number.into_inner()))
}

pub async fn delete_order(table_and_dish: web::Path<(usize, String)>,
                          state_manager: web::Data<RestaurantStateManager>)
                          -> Result<HttpResponse, RestaurantError> {
    let (table_number, dish) = table_and_dish.into_inner();
    match state_manager.delete_order(table_number, dish) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Err(RestaurantError::BadRequest(String::from("could not delete the order")))
    }
}