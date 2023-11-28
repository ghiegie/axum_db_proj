use axum::Json;
use odbc_api::{Environment, ConnectionOptions, IntoParameter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostSales {
    customer_id: i32,
    product_id: i32,
    qty: i32,
    status: String,
}

pub async fn post_sales_order(Json(body): Json<PostSales>) -> String {
    let env = Environment::new().expect("ERROR 3: FAILED TO CREATE ENVIRONMENT");
    let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";
    let conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default()).expect("ERROR 4: FAILED TO CREATE CONNECTION");

    let customer_id = body.customer_id;
    let product_id = body.product_id;
    let qty = body.qty;
    let status = body.status;

    conn.execute("insert into SalesOrderTbl(CustomerID, ProductID, ProdQty, Status) values(?, ?, ?, ?)", (&customer_id.into_parameter(), &product_id.into_parameter(), &qty.into_parameter(), &status.into_parameter())).expect("FAILED TO INSERT");

    "success".to_owned()
}