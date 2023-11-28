use axum::Json;
use odbc_api::{Environment, ConnectionOptions, IntoParameter};
use serde::{Deserialize, Serialize};

pub mod post_sales_order_mod;

pub async fn 