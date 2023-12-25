use sea_orm::{Database, DatabaseConnection};

pub struct DB {
    db_url: String,
}

impl DB {
    pub fn new(db_url: String) -> Self {
        DB { db_url }
    }

    pub async fn get_connection(&self) -> DatabaseConnection {
        Database::connect(&self.db_url).await.unwrap()
    }
}

