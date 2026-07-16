// nawy
// my first attempt at a discord bot written in rust, lord save us all

// imports :3
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
// implementation of event handler so when message contents == .ping we get a pong :3
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == ".ping"
            && let Err(why) = msg.channel_id.say(&ctx.http, "Pong :3").await {
                println!("Error sending message: {why:?}");
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

    // finafuckingly start a single shard (whatever that means) and start listening to events :3
    if let Err(why) = client.start().await {
        println!("client error: {why:?}");
    }
}
