use rocket::{State, Shutdown};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::sync::broadcast::{Sender, error::RecvError};
use rocket::tokio::select;

#[post("/message", data = "<form>")]
pub fn post(form: Form<super::super::Message>, queue: &State<Sender<super::super::Message>>) -> &'static str {
    let _res = queue.send(form.into_inner());

    "brodcaster is posting"
}

#[get("/events")]
pub async fn events(queue: &State<Sender<super::super::Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => {
                        println!("{:?}", msg);
                        msg
                    },
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}