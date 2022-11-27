use rocket::fs::{FileServer,relative};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel};
use dotenv;

mod data_source;
mod router;

#[macro_use] extern crate rocket;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..30))]
    pub username: String,

    pub message: String
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    
    let main_channel = channel::<Message>(1024). 0;

    data_source::main::init(main_channel.clone()).await;

    rocket::build()
        .manage(main_channel)
        .mount("/", routes![
            router::routes::post,
            router::routes::events
        ])
        .mount("/", FileServer::from(relative!("static")))
}