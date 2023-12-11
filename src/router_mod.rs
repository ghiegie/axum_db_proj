use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use odbc_api::{
    sys::{AttrConnectionPooling, AttrCpMatch},
    Environment,
};

use self::handler_mod::{
    get_customers_mod::get_customer,
    get_product_mod::get_product,
    post_task_mod::post_task,
    post_with_new_cust_mod::post_with_new_cust,
    post_with_old_cust_mod::post_with_old_cust,
    tasks_mod::{get_desig_mod::get_desig, get_workers_mod::get_workers},
};

pub mod handler_mod;

pub fn create_router() -> Router {
    // TODO: This connection string must be an env value
    let conn_str = String::from("Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;");

    Router::new()
        .route("/get_customer", get(get_customer))
        .route("/get_product", get(get_product))
        .route("/post_so_old_cust", post(post_with_old_cust))
        .route("/post_so_new_cust", post(post_with_new_cust))
        .route("/post_task", post(post_task))
        .nest("/tasks", task_transactions())
        .with_state((
            conn_str,
            Arc::new({
                unsafe {
                    Environment::set_connection_pooling(AttrConnectionPooling::DriverAware)
                        .expect("UNSAFE CODE ERROR ");
                }

                let mut env = Environment::new().unwrap();

                env.set_connection_pooling_matching(AttrCpMatch::Strict)
                    .expect("CREATION OF DB POOL FAILED");
                env
            }),
        ))
}

pub fn task_transactions() -> Router<(String, Arc<Environment>)> {
    Router::new()
        .route("/get_desig", get(get_desig))
        .route("/get_workers", get(get_workers))
}
