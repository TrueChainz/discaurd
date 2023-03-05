use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn create_client() -> Result<DatabaseConnection, DbErr> {
    dotenv().expect("Failed to read .env file");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let server_url = format!("{db_url}");
    let db = Database::connect(server_url).await?;

    return Ok(db);
}
