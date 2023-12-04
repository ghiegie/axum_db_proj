use axum::{extract::State, Json};
use odbc_api::{ConnectionOptions, Environment, IntoParameter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostStruct {
    customer_id: i32,
    product_id: i32,
    qty: i32,
    status: String,
}

pub async fn post_with_old_cust(
    State((con_str, env)): State<(String, Arc<Environment>)>,
    Json(body): Json<PostStruct>,
) -> String {
    let env = Arc::clone(&env);
    let conn = env
        .connect_with_connection_string(&con_str, ConnectionOptions::default())
        .expect("ERROR: FAILED TO ESTABLISH DB CONNECTION");

    let sql_query =
        "insert into SalesOrderTbl(CustomerID, ProductID, ProdQty, Status) values(?, ?, ?, ?)";

    conn.execute(
        sql_query,
        (
            &body.customer_id.into_parameter(),
            &body.product_id.into_parameter(),
            &body.qty.into_parameter(),
            &body.status.into_parameter(),
        ),
    )
    .expect("FAILED TO INSERT");

    "Success".to_owned()
}
