use crate::prisma::{self, PrismaClient};
use prisma_client_rust::NewClientError;

pub async fn create_client() -> Result<PrismaClient, NewClientError> {
    return prisma::new_client().await;
}
