use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};
use std::env;

pub async fn get_mongodb_client() -> mongodb::error::Result<Database> {
    // Replace the placeholder with your Atlas connection string
    let uri = env::var("MONGODB_URI").expect("Error: MONGODB_URI not set in env");
    let mut client_options = ClientOptions::parse(uri).await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("You successfully connected to MongoDB!");
    let collection = client.database("blog");
    Ok(collection)
}
