#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
// self imports
mod commands;

// Import types from commands module
use commands::{Data, Error, join, framework_builder};

// std library imports
use std::{env, sync::Arc};

// dotenv
use dotenvy::dotenv;

// serenity and songbird imports
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use poise::serenity_prelude as serenity;
use poise::builtins;

// tokio related imports
use tracing::{debug, error, info, span, warn, Level};



struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!pong" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "!ping").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    // Login with a bot token from the environment
    let discord_token = env::var("DISCORD_TOKEN").expect("Expected <DISCORD_TOKEN> in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = 
    GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = framework_builder().build();

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&discord_token, intents).framework(framework).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
