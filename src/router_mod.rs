use axum::{
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use odbc_api::{
    sys::{AttrConnectionPooling, AttrCpMatch},
    ConnectionOptions, Environment,
};
use std::sync::{Arc, Mutex};

use crate::router_mod::handler_mod::{get_customer_mod::get_customers, post_sales_order_with_cust_mod::post_sales_order_with_customer};
use crate::router_mod::handler_mod::{
    get_product_mod::get_product, post_sales_order_mod::post_sales_order,
};

pub mod handler_mod;

pub fn create_router() -> Router {
    lazy_static! {
        pub static ref ENV: Environment = {
            unsafe {
                Environment::set_connection_pooling(AttrConnectionPooling::DriverAware)
                    .expect("UNSAFE CODE ERROR ");
            }

            let mut env = Environment::new().unwrap();

            env.set_connection_pooling_matching(AttrCpMatch::Strict)
                .expect("CREATION OF DB POOL FAILED");
            env
        };
    }

    let con_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";
    let conn = ENV
        .connect_with_connection_string(con_str, ConnectionOptions::default())
        .unwrap();
    let conn: Arc<Mutex<odbc_api::Connection<'_>>> = Arc::new(Mutex::new(conn));

    Router::new()
        .route("/post_sales_order", post(post_sales_order))
        .route("/get_customer", get(get_customers))
        .route("/get_product", get(get_product))
        .route("/post_sales_order_with_cust", post(post_sales_order_with_customer))
        .with_state(conn)
}
