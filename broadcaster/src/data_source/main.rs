use rocket::tokio::sync::broadcast::Sender;
use rocket::tokio::spawn;
use rocket::tokio::time::{sleep, Duration};

pub fn init(main_channel: Sender<super::super::Message>) {
    spawn(async move {
        let channel = main_channel.clone();

        loop {
            sleep(Duration::from_millis(500)).await;

            let msg = super::super::Message{
                room: String::from("lobby"),
                username: String::from("datasource_init"),
                message: String::from("ping")
            };

            match channel.send(msg) {
                Ok(_v) => println!("sending a value"),
                Err(e) => println!("error parsing header: {e:?}"),
            }

            println!("should init the selected data source");
        }
    });
}