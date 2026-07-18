// nawy
// my first attempt at a discord bot written in rust, lord save us all

// imports :3
use serenity::async_trait;
use serenity::builder::EditMessage; // Required for Serenity v0.12+
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::time::Instant;

struct Handler;

#[async_trait]
// implementation of event handler so when message contents == .ping we get a pong :3
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == ".ping" {
            // start the timer
            let start_time = Instant::now();

            // initial message :3
            match msg.channel_id.say(&ctx.http, "Pinging.... :3").await {
                Ok(mut response_msg) => {
                    // calculate the time
                    let latency = start_time.elapsed().as_millis();
                    let new_content = format!("Pong! Latency is **{}ms** :3", latency);

                    let builder = EditMessage::new().content(new_content);

                    if let Err(why) = response_msg.edit(&ctx.http, builder).await {
                        println!("Error editing the message: {why:?}");
                    }
                }
                Err(why) => {
                    println!("Error sending message: {why:?}");
                }
            }
        }

        if msg.content == ".coinflip" {
            let flip = rand::random();
            let result = if flip { "Heads :3" } else { "Tails :3" };
            let _ = msg.channel_id.say(&ctx.http, result).await;
        }
        if msg.content == ".hackclub" {
            let _ = msg
                .channel_id
                .say(&ctx.http, "Wish me luck :3 ~ Energy out")
                .await;
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
