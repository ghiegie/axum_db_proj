use axum_db_proj::{router_mod::create_router, *};

#[tokio::main]
async fn main() {
    run_server(tcplistener_binder("0.0.0.0:3000").await, create_router()).await;
}
