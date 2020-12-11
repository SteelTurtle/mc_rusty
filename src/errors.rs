use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Serialize, Deserialize)]
pub enum RestaurantError {
    #[display(fmt = "Table number mismatch error")]
    TableNumberMismatchError,
    #[display(fmt = "Bad request: {}", _0)]
    BadRequest(String),
}

impl ResponseError for RestaurantError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RestaurantError::TableNumberMismatchError =>
                HttpResponse::InternalServerError().json("Error: wrong table number selected"),
            RestaurantError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
        }
    }
}