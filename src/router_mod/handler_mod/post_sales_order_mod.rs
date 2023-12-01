use axum::debug_handler;
use axum::{extract::State, Json};
use odbc_api::{ConnectionOptions, Environment, IntoParameter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostSales {
    customer_id: i32,
    product_id: i32,
    qty: i32,
    status: String,
}

#[debug_handler]
pub async fn post_sales_order(
    State(conn): State<Arc<Environment>>,
    Json(body): Json<PostSales>,
) -> String {
    let customer_id = body.customer_id;
    let product_id = body.product_id;
    let qty = body.qty;
    let status = body.status;

    let conn = Arc::clone(&conn);
    let con_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";
    let conn = conn
        .connect_with_connection_string(con_str, ConnectionOptions::default())
        .unwrap();

    conn.execute(
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
