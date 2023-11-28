use axum::Router;
use tokio::net::TcpListener;

mod model_mod;
pub mod router_mod;

pub async fn tcplistener_binder(addr: &str) -> TcpListener {
    tokio::net::TcpListener::bind(addr)
        .await
        .expect("ERROR 1: FAILED TO BIND TCPLISTENER TO IP ADDRESS")
}

pub async fn run_server(listener: TcpListener, app: Router) {
    axum::serve(listener, app)
        .await
        .expect("ERROR 2: FAILED TO SERVE THE GIVEN AXUM SERVER");
}
