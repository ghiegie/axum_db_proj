use std::sync::{Arc, Mutex};

use axum::{extract::State, Json};
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer, Item},
    IntoParameter, Cursor,
};
use serde::{Serialize, Deserialize};

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

pub async fn post_sales_order_with_customer(
    State(conn): State<Arc<Mutex<odbc_api::Connection<'_>>>>,
    Json(body): Json<PostSales>,
) -> String {
    conn.lock().unwrap().execute(
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

    let cust_id: Option<i32> = {
        let batch_size = 5;
        let buffer_description = [BufferDesc::Text { max_str_len: 255 }];
        let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_description);

        let query = format!(
            "select CustomerID from CustomerTbl where Name = '{}' and Addr = '{}' and TIN = '{}' and TelNo = '{}' and Email = '{}' and ContPers = '{}' and DelivAddr = '{}'", 
            &body.customer_name, &body.customer_addr, &body.tin, &body.tel_no, &body.email, &body.cont_pers, &body.del_addr
        );

        if let Some(cursor) = conn
            .lock()
            .expect("FAILED TO UNLOCK MUTEX")
            .execute(&query, ())
            .expect("FAILED TO EXECUTE QUERY")
        {
            // bind cursor to buffer
            let mut row_set_cursor = cursor
                .bind_buffer(&mut buffer)
                .expect("FAILED TO BIND CURSOR TO BUFFER");

            let row_set = row_set_cursor.fetch().unwrap();
            let a = row_set.unwrap().column(0).as_text_view().unwrap().len();
            println!("{}", a);
            Some(1)
        } else {
            None
        }
    };


    cust_id.unwrap().to_string()
}
