use rocket::tokio::sync::broadcast::Sender;
use super::mongo;

// use rocket::tokio::time::{sleep, Duration};

pub async fn init(_main_channel: Sender<super::super::Message>) {
    mongo::init_mongo_connection().await.unwrap();
}