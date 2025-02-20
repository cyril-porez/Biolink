use axum::{Router, routing::{get, post}, Json, extract::Path, extract::Query};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;


#[utoipa::path(
  get,
  path = '/',
  responses(
    (status = 200, description = "Message hello world", body = String)
  )
)]
async fn hello() -> &'static str {
  "Hello World !"
}

#[derive(OpenApi)]
#[openapi(paths(hello))]
struct  ApiDoc;

pub fn create_routes() -> Router { 
  Router::new()
    .route("/", get(hello))
    .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}