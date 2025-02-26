use axum::{routing::connect, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod routes;
use crate::routes::create_routes;
mod database;
use crate::database::connect_db;
use crate::database::get_users;

#[tokio::main]
async fn main() {

    let client = connect_db().await.expect("Impossible de se connecter à PostgreSQL");
    let users = get_users(&client).await.expect("Erreur lors de la récupération des utilisateurs");
    println!("Utilisateurs : {:?}", users.get(0));
    println!("Utilisateurs : {:?}", users.get(1));
    let mut test  = 0;
    if let Some((id,_,_)) = users.get(0) {
        test = *id;
    }
    println!("Test : {}", test);
    

    let app = create_routes();


    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server lancé sur http://{}", listener.local_addr().unwrap());
    println!("Swagger UI disponible sur http://127.0.0.1:3000/docs");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

}
