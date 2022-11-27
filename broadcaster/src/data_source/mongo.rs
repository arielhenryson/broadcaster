use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

pub async fn init_mongo_connection() -> mongodb::error::Result<()> {
    println!("init default datasource mongoDB");

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");


    // Parse your connection string into an options struct
    let mut client_options = ClientOptions::parse(client_uri)
         .await?;

    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster

    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("Connected successfully.");
    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}