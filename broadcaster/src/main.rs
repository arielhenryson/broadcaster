use rocket::State;
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender};
use rocket::form::Form;

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

#[get("/")]
fn index() -> &'static str {
    "brodcaster is running"
}

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) -> &'static str {

    let _res = queue.send(form.into_inner());
    "brodcaster is posting"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024). 0)
        .mount("/", routes![
            index,
            post
        ])
}