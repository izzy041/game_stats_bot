use std::env;
use serenity::all::GatewayIntents;
use serenity::Client;

mod handler;

#[tokio::main]
async fn main() {
    // Get the discord developer token from environment variable.
    let token: String = env::var("DISCORD_TOKEN").expect("token not provided");

    // Set the type of events to listen to.
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Try to create a client instance,
    // print the error if it fails.
    let mut client: Client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("could not create client");

    // Start the client,
    // print the error if it fails.
    if let Err(err) = client.start().await {
        println!("client error {err}")
    }
}
