use rocket::fs::{FileServer,relative};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel};

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
fn rocket() -> _ {
    data_source::main::init();

    rocket::build()
        .manage(channel::<Message>(1024). 0)
        .mount("/", routes![
            router::routes::post,
            router::routes::events
        ])
        .mount("/", FileServer::from(relative!("static")))
}