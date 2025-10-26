use sea_orm::{Database, DatabaseConnection};

pub async fn init_db() -> DatabaseConnection{
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");
    Database::connect(&database_url).await.expect("Could not connect to database")
}
