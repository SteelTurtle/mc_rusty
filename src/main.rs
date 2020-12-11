use actix_web::{App, HttpServer, web};

use mc_rusty::handlers::{add_order, delete_order, get_orders};
use mc_rusty::restaurant_manager::RestaurantStateManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set console logging
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let state_manager = web::Data::new(RestaurantStateManager::instance());

    HttpServer::new(move || {
        App::new()
            .app_data(state_manager.clone())
            .data(web::JsonConfig::default().limit(4096))
            // everything under '/api/' route
            .service(
                web::scope("/api")
                    .route("/orders/{table_num}", web::post().to(add_order))
                    .route("/orders/{table_num}", web::get().to(get_orders))
                    .route("/orders/{table_num}/{dish_name}", web::delete().to(delete_order)),
            )
    }).bind("127.0.0.1:9090")?
        .run()
        .await
}
