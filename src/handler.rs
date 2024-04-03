use async_trait::async_trait;
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter, CreateMessage, Message, Ready, Timestamp};
use serenity::prelude::{Context, EventHandler};

mod response;
mod commands;

// The event handler.
pub struct Handler {
    pub client: reqwest::Client,
}
#[async_trait]
impl EventHandler for Handler {
    // Create a handler for the 'message' event - the function is called when a message is received
    async fn message(&self, ctx: Context, msg: Message) {
        let prefix: &str = "!";

        if msg.content == format!("{prefix}ping").to_string() {
            commands::ping(&msg.channel_id, &ctx).await;

        } else if msg.content.chars().count() >= 10 && msg.content[0..10] == format!("{prefix}statsapex").to_string() {
            let args: Vec<&str> = msg.content.split(' ').collect();

            // Check if the correct amount of arguments was provided.
            if args.len() != 3 {
                // Create and send a message with an embed explaining the issue.
                let missing_args_embed = CreateEmbed::new()
                    .title("Error getting Apex Legends Statistics")
                    .description("Missing command arguments, usage: !statsapex <player> <platform>")
                    .footer(CreateEmbedFooter::new(format!("Apex legends statistics requested by {}.", &msg.author.name)))
                    .timestamp(Timestamp::now())
                    .color(Colour::RED);
                if let Err(err) = msg.channel_id.send_message(ctx, CreateMessage::new().embed(missing_args_embed)).await {
                    println!("failed to send message: {err}");
                }
            } else {
                // Get the player name and platform from the arguments provided.
                let player: &str = &args.get(1).unwrap_or(&"");
                let mut platform: &str = &args.get(2).unwrap_or(&"").to_uppercase();
                let plats: Vec<&str> = vec!["PC", "XBOX", "PLAYSTATION"];

                // Check if a valid platform was provided.
                if plats.contains(&platform.to_uppercase().as_str()) {
                    // Convert the platform to one that is accepted by the API.
                    platform = {
                        match platform {
                            "XBOX" => "X1",
                            "PLAYSTATION" => "PS4",
                            _ => "PC"
                        }
                    };

                    // Get the player stats.
                    commands::get_apex_stats(&player.to_string(), &msg.channel_id, &self.client, &ctx, msg.author, &platform.to_string().to_uppercase()).await;
                } else {
                    // If a valid platform was not provided, show the proper
                    // command usage.
                    let incorrect_platform_embed = CreateEmbed::new()
                        .title("Error getting Apex Legends Statistics")
                        .description("Incorrect platform specified, available platforms are `pc`, `xbox` or `playstation`")
                        .footer(CreateEmbedFooter::new(format!("Apex legends statistics requested by {}.", &msg.author.name)))
                        .timestamp(Timestamp::now())
                        .color(Colour::RED);
                    if let Err(err) = msg.channel_id.send_message(ctx, CreateMessage::new().embed(incorrect_platform_embed)).await {
                        println!("failed to send message: {err}");
                    }
                }
            }
        }
    }

    // Function to handle the 'ready' event which is called when the bot is started.
    // Receives data containing information about the bot in the 'ready' argument.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is online", ready.user.name);
    }
}
