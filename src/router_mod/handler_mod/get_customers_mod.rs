use axum::{extract::State, Json};
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer},
    ConnectionOptions, Cursor, Environment,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetCust {
    list: Vec<CustData>,
}

impl GetCust {
    pub fn new(list: Vec<CustData>) -> Self {
        Self { list }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustData {
    id: i32,
    name: String,
}

impl CustData {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

pub async fn get_customer(
    State((con_str, env)): State<(String, Arc<Environment>)>,
) -> Json<GetCust> {
    let env = Arc::clone(&env);

    let conn = env
        .connect_with_connection_string(&con_str, ConnectionOptions::default())
        .expect("ERROR: FAILED TO ESTABLISH DB CONNECTION");

    let sql_query = "select CustomerID, Name from CustomerTbl";

    let batch_size = 1000; // size of buffer
    let buffer_desc = [
        BufferDesc::I32 { nullable: false },
        BufferDesc::Text { max_str_len: 50 }, // max length of string to get
    ];
    let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_desc);

    let cursor = conn
        .execute(sql_query, ())
        .expect("ERROR: FAILED TO CREATE CURSOR")
        .expect("ERROR: CREATED CURSOR IS EMPTY");

    let mut buffer_cursor = cursor
        .bind_buffer(&mut buffer)
        .expect("ERROR: FAILED TO CREATE BUFFER-CURSOR");

    if let Some(fetch_data) = buffer_cursor.fetch().expect("ERROR: FAILED TO FETCH") {
        let col = fetch_data.column(0);
        let id_arr = col.as_slice::<i32>().unwrap().to_vec();

        let mut name_vec = Vec::new();
        let col = fetch_data.column(1);
        if let Some(a) = col.as_text_view() {
            // shows the raw textcolview stream
            for b in a.iter() {
                // converts the stream to an iterable (chopping the stream)
                if let Some(c) = b {
                    // returns a stream from option
                    // from_utf8 takes a vector to turn it into a string
                    // c is an array not vec so we convert it
                    let my_str = String::from_utf8(c.to_vec()).unwrap();
                    name_vec.push(my_str)
                }
            }
        }

        let mut list = Vec::new();
        for (id, name) in id_arr.into_iter().zip(name_vec.into_iter()) {
            list.push(CustData::new(id, name))
        }

        return Json(GetCust::new(list));
    }

    Json(GetCust::new(Vec::new()))
}
