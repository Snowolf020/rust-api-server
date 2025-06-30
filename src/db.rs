use prisma::PrismaClient;
use prisma::models::user;
use std::env;

#[derive(Debug)]
pub struct Database {
    client: PrismaClient,
}

impl Database {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let client = PrismaClient::new(database_url);
        Database { client }
    }

    pub async fn create_user(&self, name: String, email: String) -> user::Data {
        self.client
            .user()
            .create(vec![user::UserCreateInput {
                name: Some(name),
                email: Some(email),
                ..Default::default()
            }])
            .exec()
            .await
            .expect("Failed to create user")
    }

    pub async fn get_user(&self, id: i32) -> Option<user::Data> {
        self.client.user().find(id).exec().await.ok()
    }

    pub async fn get_all_users(&self) -> Vec<user::Data> {
        self.client.user().find_many(vec![]).exec().await.unwrap()
    }
}