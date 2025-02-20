use axum::{Router, routing::{get, post}, Json, extract::Path, extract::Query};
use serde::{Deserialize, Serialize};

async fn hello() -> &'static str {
  "Hello World !"
}

pub fn create_routes() -> Router { 
  Router::new()
    .route("/", get(hello))
}