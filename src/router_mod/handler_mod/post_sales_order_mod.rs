use axum::{extract::State, Json};
use odbc_api::IntoParameter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostSales {
    customer_id: i32,
    product_id: i32,
    qty: i32,
    status: String,
}

pub async fn post_sales_order(
    State(conn): State<Arc<Mutex<odbc_api::Connection<'_>>>>,
    Json(body): Json<PostSales>,
) -> String {
    let customer_id = body.customer_id;
    let product_id = body.product_id;
    let qty = body.qty;
    let status = body.status;

    conn.lock()
        .unwrap()
        .execute(
            "insert into SalesOrderTbl(CustomerID, ProductID, ProdQty, Status) values(?, ?, ?, ?)",
            (
                &customer_id.into_parameter(),
                &product_id.into_parameter(),
                &qty.into_parameter(),
                &status.into_parameter(),
            ),
        )
        .expect("FAILED TO INSERT");

    "success".to_owned()
}
