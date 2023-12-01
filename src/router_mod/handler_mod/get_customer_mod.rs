use axum::debug_handler;
use axum::{extract::State, Json};
use odbc_api::ConnectionOptions;
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer},
    Cursor, Environment,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct GetResponse {
    customer_names: Vec<String>,
}

impl GetResponse {
    pub fn new(list: Vec<String>) -> Self {
        Self {
            customer_names: list,
        }
    }
}

#[debug_handler]
pub async fn get_customers(State(conn): State<Arc<Environment>>) -> Json<GetResponse> {
    let batch_size = 1000;
    let buffer_description = [BufferDesc::Text { max_str_len: 255 }];
    let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_description);

    let conn = Arc::clone(&conn);
    let con_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";
    let conn = conn
        .connect_with_connection_string(con_str, ConnectionOptions::default())
        .unwrap();

    let query = "select Name from CustomerTbl";
    let mut response = Vec::new();
    if let Some(cursor) = conn
        .execute(query, ())
        .expect("FAILED TO CREATE CURSOR FROM QUERY")
    {
        let mut row_set_cursor = cursor
            .bind_buffer(&mut buffer)
            .expect("FAILED TO BIND CURSOR TO BUFFER");

        while let Some(row_set) = row_set_cursor.fetch().expect("FAILED TO FETCH") {
            let col = row_set.column(0);
            if let Some(a) = col.as_text_view() {
                for b in a.iter() {
                    if let Some(c) = b {
                        response.push(String::from_utf8(c.to_vec()).unwrap());
                    }
                }
            }
        }
    }

    println!("GET CUSTOMER CREATED");
    Json(GetResponse::new(response))
}
