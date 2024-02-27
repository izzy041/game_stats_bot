use reqwest::Client;
use serenity::all::ChannelId;
use serenity::prelude::Context;
use crate::handler::response;

pub async fn get_apex_stats (player_name: &String, channel: &ChannelId, http_client: &Client, ctx: &Context) {
    let response = response::get_response(&http_client, player_name).await;
    println!("player is {player_name}");

    // If the request failed, print the error.
    if let Err(err) = response {
        println!("request failed: {err}");
    }
    // If the request succeeded, print the result.
    else {
        // If the response is too big to send in a single message,
        // split the message int multiple smaller parts.
        // Otherwise, send the message.
        let response_body: String = response.unwrap();
        if response_body.len() > 1500 {
            for string in response::split_response(&response_body) {
                if let Err(err) = channel.say(&ctx.http, string).await {
                    println!("failed to send message: {err}");
                }
            }
        } else {
            if let Err(err) = channel.say(&ctx.http, response_body).await {
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