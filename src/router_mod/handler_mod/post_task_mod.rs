use axum::{extract::State, Json};
use odbc_api::{sys::Date, ConnectionOptions, Environment, IntoParameter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostTask {
    task: String,
}

pub async fn post_task(
    State((con_str, env)): State<(String, Arc<Environment>)>,
    Json(body): Json<PostTask>,
) -> String {
    let env = Arc::clone(&env);
    let conn = env
        .connect_with_connection_string(&con_str, ConnectionOptions::default())
        .expect("ERROR: FAILED TO ESTABLISH DB CONNECTION");

    let sql_query = "insert into SampleTaskTbl(Task, TaskDate) values(?, ?)";

    let date = Date {
        year: 2023,
        month: 12,
        day: 7,
    };

    conn.execute(
        sql_query,
        (&body.task.into_parameter(), &date.into_parameter()),
    )
    .expect("FAILED TO INSERT");

    let now = OffsetDateTime::now_local();

    format!("{:?}", now)
}
