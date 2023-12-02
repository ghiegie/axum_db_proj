use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use odbc_api::{
    sys::{AttrConnectionPooling, AttrCpMatch},
    Environment,
};

use crate::router_mod::handler_mod::{
    get_customer_mod::get_customers, post_sales_order_with_cust_mod::post_sales_order_with_customer,
};
use crate::router_mod::handler_mod::{
    get_product_mod::get_product, post_sales_order_mod::post_sales_order,
};

pub mod handler_mod;

pub fn create_router() -> Router {
    Router::new()
        .route("/post_sales_order", post(post_sales_order))
        .route("/get_customer", get(get_customers))
        .route("/get_product", get(get_product))
        .route(
            "/post_sales_order_with_cust",
            post(post_sales_order_with_customer),
        )
        .with_state(Arc::new({
            unsafe {
                Environment::set_connection_pooling(AttrConnectionPooling::DriverAware)
                    .expect("UNSAFE CODE ERROR ");
            }

            let mut env = Environment::new().unwrap();

            env.set_connection_pooling_matching(AttrCpMatch::Strict)
                .expect("CREATION OF DB POOL FAILED");
            env
        }))
}
