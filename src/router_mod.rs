use axum::{Router, routing::{get, post}};

use self::handler_mod::post_sales_order_mod::post_sales_order;


pub mod handler_mod;

pub fn create_router() -> Router {
    Router::new()
        .route("/post_sales_order", post(post_sales_order))
}