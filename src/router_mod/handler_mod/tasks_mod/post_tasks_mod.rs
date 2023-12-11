use axum::{extract::State, Json};
use odbc_api::{ConnectionOptions, Environment, IntoParameter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostStruct {
    task: String,
    start_date: String,
    start_time: String,
    status: String,
    desig_id: i32,
    worker_id: i32,
}

pub async fn post_task(
    State((con_str, env)): State<(String, Arc<Environment>)>,
    Json(body): Json<PostStruct>,
) -> String {
    let env = Arc::clone(&env);
    let conn = env
        .connect_with_connection_string(&con_str, ConnectionOptions::default())
        .expect("ERROR: FAILED TO ESTABLISH DB CONNECTION");

    let sql_query = "insert into SampleTasksTbl(Task, TaskStartDate, TaskStartTime, Status, DesignationID, WorkerID) values(?, ?, ?, ?, ?, ?)";

    conn.execute(
        sql_query,
        (
            &body.task.into_parameter(),
            &body.start_date.into_parameter(),
            &body.start_time.into_parameter(),
            &body.status.into_parameter(),
            &body.desig_id.into_parameter(),
            &body.worker_id.into_parameter(),
        ),
    )
    .expect("FAILED TO INSERT");

    "Success".to_owned()
}
