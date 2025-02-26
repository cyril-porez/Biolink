use tokio_postgres::{NoTls, Error};
use dotenvy::dotenv;
use core::error;
use std::env;

pub async fn connect_db() -> Result<tokio_postgres::Client, Error> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL")
                          .expect("DATABASE_URL doit être défini dans .env");

  let (client, connection) = 
      tokio_postgres::connect(&database_url, NoTls).await?;

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("Erreur de connexion PostgreSQL: {}", e);
    }
  });

  println!("Connecté à PostgreSql !");
  Ok(client)

}

pub async fn get_users(client: &tokio_postgres::Client) -> Result<Vec<(i32, String, String)>, Error> {
  let rows = client.query("SELECT * FROM users", &[]).await?;
  let users = rows.iter()
    .map(|row| {
      let id: i32 = row.get(0);
      let username: String = row.get(1);
      let email: String = row.get(2);
      (id, username, email)
    })
    .collect();
  Ok(users)
}
