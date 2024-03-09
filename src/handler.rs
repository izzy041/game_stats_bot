use async_trait::async_trait;
use serenity::all::{Message, Ready};
use serenity::prelude::{Context, EventHandler};

mod response;
mod commands;

// The event handler.
pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    // Create a handler for the 'message' event - the function is called when a message is received
    async fn message(&self, ctx: Context, msg: Message) {
        let prefix: &str = "!";

        // Reuse the same client instead of creating a new one
        // every time a request is made.
        let http_client: reqwest::Client = reqwest::Client::new();

        if msg.content == format!("{prefix}ping").to_string() {
            commands::ping(&msg.channel_id, &ctx).await;

        } else if msg.content.chars().count() > 10 && msg.content[0..10] == format!("{prefix}statsapex").to_string() {
            let player: &String = &msg.content[11..].to_string();
            commands::get_apex_stats(player, &msg.channel_id, &http_client, &ctx).await;
        }
    }

    // Function to handle the 'ready' event which is called when the bot is started.
    // Receives data containing information about the bot in the 'ready' argument.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is online", ready.user.name);
    }
}
