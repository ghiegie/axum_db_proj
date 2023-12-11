use axum::{extract::State, Json};
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer},
    ConnectionOptions, Cursor, Environment,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerData {
    id: i32,
    worker: String,
}

impl WorkerData {
    pub fn new(id: i32, worker: String) -> Self {
        Self { id, worker }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetWorker {
    list: Vec<WorkerData>,
}

impl GetWorker {
    pub fn new(list: Vec<WorkerData>) -> Self {
        Self { list }
    }
}

pub async fn get_workers(
    State((con_str, env)): State<(String, Arc<Environment>)>,
) -> Json<GetWorker> {
    let env = Arc::clone(&env);
    let conn = env
        .connect_with_connection_string(&con_str, ConnectionOptions::default())
        .expect("ERROR: FAILED TO ESTABLISH DB CONNECTION");

    let sql_query = "select * from SampleWorkersTbl";

    let batch_size = 100; // size of buffer
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

        let mut worker_vec = Vec::new();
        let col = fetch_data.column(1);

        if let Some(a) = col.as_text_view() {
            for b in a.iter() {
                if let Some(c) = b {
                    let my_str = String::from_utf8(c.to_vec()).unwrap();
                    worker_vec.push(my_str)
                }
            }
        }

        let mut list = Vec::new();
        for (id, worker) in id_arr.into_iter().zip(worker_vec.into_iter()) {
            list.push(WorkerData::new(id, worker))
        }

        return Json(GetWorker::new(list));
    }

    Json(GetWorker::new(Vec::new()))
}
