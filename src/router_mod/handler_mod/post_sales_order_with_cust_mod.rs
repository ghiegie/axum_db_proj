use axum::debug_handler;
use axum::{extract::State, Json};
use odbc_api::{Environment, ConnectionOptions};
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer, Item},
    Cursor, IntoParameter,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
pub struct PostSales {
    customer_name: String,
    customer_addr: String,
    tin: String,
    tel_no: String,
    email: String,
    cont_pers: String,
    del_addr: String,
    product_id: i32,
    qty: i32,
    status: String,
}

#[debug_handler]
pub async fn post_sales_order_with_customer(
    State(conn): State<Arc<Environment>>,
    Json(body): Json<PostSales>,
) -> String {

    let conn = Arc::clone(&conn);
    let con_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";
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
            &body.product_id.clone().into_parameter(),
            &body.qty.clone().into_parameter(),
            &body.status.clone().into_parameter(),
        )
    ).expect("FAILED TO EXECUTE QUERY");

    // let cust_id: Option<i32> = {
    //     let batch_size = 5;
    //     let buffer_description = [BufferDesc::I32 { nullable: false }];
    //     let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_description);

    //     let query = format!(
    //         "select CustomerID from CustomerTbl where Name = '{}' and Addr = '{}' and TIN = '{}' and TelNo = '{}' and Email = '{}' and ContPers = '{}' and DelivAddr = '{}'",
    //         &body.customer_name, &body.customer_addr, &body.tin, &body.tel_no, &body.email, &body.cont_pers, &body.del_addr
    //     );

    //     if let Some(cursor) = conn
    //         .execute(&query, ())
    //         .expect("FAILED TO EXECUTE QUERY")
    //     {
    //         // bind cursor to buffer
    //         let mut buffer_with_cursor = cursor.bind_buffer(&mut buffer).unwrap();
    //         // fetch row_sets
    //         let row_set = buffer_with_cursor
    //             .fetch()
    //             .expect("Error in creating row_set")
    //             .expect("No value returned");
    //         // using .column()? maybe this is accessing certain column of a row set
    //         let refined_rowset = row_set.column(0);

    //         // this is for accessing the values of the specific column in a row set. this varies for each type

    //         // converting the rowset into int rowsets
    //         match i32::as_slice(refined_rowset) {
    //             Some(a) => {
    //                 println!("{}", a.len() as i32);
    //             }
    //             None => {
    //                 println!("{}", "NO RESULT");
    //             }
    //         }
    //     }

    //     Some(1)
    // };

    "test".to_owned()
}
