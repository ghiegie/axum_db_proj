use axum::debug_handler;
use axum::{extract::State, Json};
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer, Item},
    Cursor, IntoParameter,
};
use odbc_api::{ConnectionOptions, Environment};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct PostSales {
    customer_name: String,
    customer_addr: String,
    tin: String,
    tel_no: String,
    email: String,
    cont_pers: String,
    del_addr: String,
    prd_id: i32,
    qty: i32,
    stat: String,
}

#[debug_handler]
pub async fn post_sales_order_with_customer(
    State(conn): State<Arc<Environment>>,
    Json(body): Json<PostSales>,
) {
    let conn = Arc::clone(&conn);
    let mut con_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";
    let conn = conn
        .connect_with_connection_string(con_str, ConnectionOptions::default())
        .unwrap();

    conn.execute(
        "insert into CustomerTbl(Name, Addr, TIN, TelNo, Email, ContPers, DelivAddr) values(?, ?, ?, ?, ?, ?, ?)", (
            &body.customer_name.clone().into_parameter(),
            &body.customer_addr.clone().into_parameter(),
            &body.tin.clone().into_parameter(),
            &body.tel_no.clone().into_parameter(),
            &body.email.clone().into_parameter(),
            &body.cont_pers.clone().into_parameter(),
            &body.del_addr.clone().into_parameter(),
        )
    ).expect("FAILED TO EXECUTE QUERY");

    let cust_id: i32 = {
        let batch_size = 5;
        let buffer_desc = [BufferDesc::I32 { nullable: false }];
        let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_desc);

        let query = format!(
            "select CustomerID from CustomerTbl where Name = '{}' and Addr = '{}' and TIN = '{}' and TelNo = '{}' and Email = '{}' and ContPers = '{}' and DelivAddr = '{}'",
            &body.customer_name, &body.customer_addr, &body.tin, &body.tel_no, &body.email, &body.cont_pers, &body.del_addr
        );

        let cursor = conn
            .execute(&query, ())
            .expect("FAILED TO RETRIEVE CUST ID")
            .expect("CURSOR NOT CREATED");
        let mut buffer_with_cursor = cursor.bind_buffer(&mut buffer).unwrap();
        let row_set = buffer_with_cursor
            .fetch()
            .expect("FAILED TO FETCH")
            .expect("FETCH RETURNED NONE");
        let refined_rowset = row_set.column(0);

        i32::as_slice(refined_rowset).expect("NO CUST ID RETURNED")[0]
    };

    con_str =
        "insert into SalesOrderTbl(CustomerID, ProductID, ProdQty, Status) values(?, ?, ?, ?)";
    conn.execute(
        con_str,
        (
            &cust_id.into_parameter(),
            &body.prd_id.into_parameter(),
            &body.qty.into_parameter(),
            &body.stat.into_parameter(),
        ),
    )
    .expect("FAILED TO EXECUTE QUERY");
}
