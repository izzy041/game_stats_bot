use reqwest::Client;
use serenity::all::{ChannelId, User};
use serenity::prelude::Context;
use crate::handler::response;

pub async fn get_apex_stats (player_name: &String, channel: &ChannelId, http_client: &Client, ctx: &Context, requester: User, platform: &String) {
    let response = response::get_response(&http_client, &player_name, &platform).await;

    // If the request failed, print the error.
    if let Err(err) = response {
        println!("request failed: {err}");
    }
    // If the request succeeded, print the result.
    else {
        let response_body: String = response.unwrap(); // Safe to unwrap since we know the request succeeded
        let parsed_response: Result<response::PlayerStats, serde_json::Error> = response::parse_json(&response_body.as_str());
        // If parsing failed, let the user know what happened.
        // Otherwise, send a message containing the parsed response.
        if parsed_response.is_err() {
            if let Err(err) = channel.say(
                &ctx.http, format!("Could not process stats: {:?}", parsed_response.err().unwrap())).await {
                println!("failed to send message: {err}");
            }
        } else {
            if let Err(err) = channel.send_message(&ctx.http, parsed_response.unwrap().as_message(&requester)).await {
                println!("failed to send message: {err}");
            }
        }
    }
}

pub async fn ping(channel: &ChannelId, ctx: &Context) {
    // Check if sending the message failed and print the reason.
    if let Err(err) = channel.say(&ctx.http, "pong").await {
        println!("could not send message: {err}")
    }
}