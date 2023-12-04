use axum::{extract::State, Json};
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer, Item},
    ConnectionOptions, Cursor, Environment, IntoParameter,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostStruct {
    customer_id: i32,
    product_id: i32,
    qty: i32,
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostCustStruct {
    cust_name: String,
    cust_addr: String,
    tin: String,
    tel_no: String,
    email: String,
    cont_pers: String,
    del_addr: String,
    prd_id: i32,
    qty: i32,
    stat: String,
}

pub async fn post_with_new_cust(
    State((con_str, env)): State<(String, Arc<Environment>)>,
    Json(body): Json<PostCustStruct>,
) {
    let env = Arc::clone(&env);
    let conn = env
        .connect_with_connection_string(&con_str, ConnectionOptions::default())
        .expect("ERROR: FAILED TO ESTABLISH DB CONNECTION");

    let mut sql_query = "insert into CustomerTbl(Name, Addr, TIN, TelNo, Email, ContPers, DelivAddr) values(?, ?, ?, ?, ?, ?, ?)";

    conn.execute(
        sql_query,
        (
            &body.cust_name.clone().into_parameter(),
            &body.cust_addr.clone().into_parameter(),
            &body.tin.clone().into_parameter(),
            &body.tel_no.clone().into_parameter(),
            &body.email.clone().into_parameter(),
            &body.cont_pers.clone().into_parameter(),
            &body.del_addr.clone().into_parameter(),
        ),
    )
    .expect("FAILED TO INSERT");

    let cust_id: i32 = {
        let batch_size = 1;
        let buffer_desc = [BufferDesc::I32 { nullable: false }];
        let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_desc);

        let query = format!(
            "select CustomerID from CustomerTbl where Name = '{}' and Addr = '{}' and TIN = '{}' and TelNo = '{}' and Email = '{}' and ContPers = '{}' and DelivAddr = '{}'",
            &body.cust_name, &body.cust_addr, &body.tin, &body.tel_no, &body.email, &body.cont_pers, &body.del_addr
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

    sql_query =
        "insert into SalesOrderTbl(CustomerID, ProductID, ProdQty, Status) values(?, ?, ?, ?)";
    conn.execute(
        sql_query,
        (
            &cust_id.into_parameter(),
            &body.prd_id.into_parameter(),
            &body.qty.into_parameter(),
            &body.stat.into_parameter(),
        ),
    )
    .expect("FAILED TO EXECUTE QUERY");
}
