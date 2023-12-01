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
    product_names: Vec<String>,
}

impl GetResponse {
    pub fn new(list: Vec<String>) -> Self {
        Self {
            product_names: list,
        }
    }
}

#[debug_handler]
pub async fn get_product(State(conn): State<Arc<Environment>>) -> Json<GetResponse> {
    let batch_size = 1000;
    let buffer_description = [BufferDesc::Text { max_str_len: 255 }];
    let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_description);
    
    let conn = Arc::clone(&conn);
    let con_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";
    let conn = conn
        .connect_with_connection_string(con_str, ConnectionOptions::default())
        .unwrap();

    let query = "select Product from ProductTbl";
    let mut response = Vec::new();
    if let Some(cursor) = conn.execute(&query, ()).expect("FAILED TO CREATE CURSOR FROM QUERY") {
        let mut buffer_with_cursor = cursor.bind_buffer(&mut buffer).unwrap();
        while let Some(row_set) = buffer_with_cursor.fetch().expect("FAILED TO FETCH") {
            let col = row_set.column(0);
            if let Some(text_col) = col.as_text_view() {
                for b in text_col.iter() {
                    if let Some(byte_arr) = b {
                        response.push(String::from_utf8(byte_arr.to_vec()).unwrap())
                    }
                }
            }
        }
    }    

    Json(GetResponse::new(response))
}
