use crate::mongo::models::WalletHistory;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;

pub mod models;

#[derive(Clone)]
pub struct Connection {
    client: mongodb::Client,
    database: Option<mongodb::Database>,
}

impl Connection {
    pub async fn new(uri: &str) -> Self {
        Connection {
            client: mongodb::Client::with_uri_str(uri)
                .await
                .expect("Database connection failed!"),
            database: None,
        }
    }

    pub fn database(&mut self, name: &str) -> &mut Connection {
        self.database = Some(self.client.database(name));
        self
    }

    pub async fn insert(&self, data: &WalletHistory) -> InsertOneResult {
        if self.database.is_none() {
            panic!("Database not set! Use client.database() to set a database")
        }

        let collection = self
            .database
            .as_ref()
            .unwrap()
            .collection::<WalletHistory>("wallethistory");

        collection
            .insert_one(data, None)
            .await
            .expect("Failed to insert wallet history")
    }

    pub async fn history<T>(&self, address: T, token: T, block: i64) -> Option<WalletHistory>
    where
        T: Into<String>,
    {
        if self.database.is_none() {
            panic!("Database not set! Use client.database() to set a database")
        }

        let collection = self
            .database
            .as_ref()
            .unwrap()
            .collection::<WalletHistory>("wallethistory");

        collection
            .find_one(
                doc! {
                    "block": block,
                    "token": token.into(),
                    "address": address.into(),
                },
                None,
            )
            .await
            .expect("Failed fetching data from database")
    }
}
