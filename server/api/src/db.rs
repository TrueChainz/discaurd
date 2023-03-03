// use crate::prisma::{self, PrismaClient};
// use prisma_client_rust::NewClientError;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn create_client() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect("mysql://dlfrdozr5hyjxfwzyyjf:pscale_pw_nm7pmhYLccmzYTWVbojGIijIFl9G1WTAkBUZJMtifjn@aws-eu-west-2.connect.psdb.cloud/discaurd?sslaccept=strict").await?;

    return Ok(db);
}
