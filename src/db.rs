use sea_orm::{Database, DatabaseConnection};
use std::env;


pub async fn connect() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");  // ✅ Read from .env
    Database::connect(database_url)
        .await
        .expect("Failed to connect to database")
}