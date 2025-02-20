use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod routes;
use crate::routes::create_routes;

#[tokio::main]
async fn main() {
    let app = create_routes();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server lancé sur http://{}", listener.local_addr().unwrap());
    println!("Swagger UI disponible sur http://127.0.0.1:3000/docs");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
