// nawy
// my first attempt at a discord bot written in rust, lord save us all

// imports :3
use serenity::async_trait; // Required for Serenity v0.12+
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
struct Handler;
mod ping;
use ping::ping;
mod coinflip;
use coinflip::coinflip;
mod hackclub;
use hackclub::hackclub;
mod help;
use help::help;
mod randcat;
use randcat::cat;
mod time;
use time::time;

#[async_trait]
// implementation of event handler so when message contents == something -> then something happens
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // ping command
        if msg.content == ".ping" {
            ping(&ctx, &msg).await;
        }
        // coinflip command
        if msg.content == ".coinflip" {
            coinflip(&ctx, &msg).await;
        }
        // hackclub command
        if msg.content == ".hackclub" {
            hackclub(&ctx, &msg).await;
        }

        // automeower :3
        // meow list :p
        let meows = vec!["meow", "nya", "mrrrp", "prr", "purr"];

        if msg.author.id == 1527332908287656036 {
        } else {
            // the thing that checks if message is meowing :3
            if meows.iter().any(|e| msg.content.contains(e)) {
                let _ = msg.channel_id.say(&ctx.http, "meow:3c").await;
            }
        }

        if msg.content == ".cat" {
            cat(&ctx, &msg).await;
        }

        if msg.content == ".help" {
            help(&ctx, &msg).await;
        }

        if msg.content == ".time" {
            time(&ctx, &msg).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected :3", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    // config client with the discord bot token in the .env
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("expected a token in the .env");

    // set gateway intents or big discord won't be happy
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // create new instance of client logging in as the bot
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating a client");

    // finafuckingly start a single shard and start listening to events :3
    if let Err(why) = client.start().await {
        println!("client error: {why:?}");
    }
}
