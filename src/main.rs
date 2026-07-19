// nawy
// my first attempt at a discord bot written in rust, lord save us all

// imports :3
use serenity::async_trait;
use serenity::builder::EditMessage; // Required for Serenity v0.12+
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
use std::time::Instant;

struct Handler;

#[async_trait]
// implementation of event handler so when message contents == something -> then something happens
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let message: &str = &msg.content;
        if message.starts_with(".") {
            match &message[1..] {
                "help" => {
                    let _ = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            "\
                                List of commands :3c
                            - .ping -> pings server
                            - .coinflip -> does a coinflip :3
                            - .cat -> returns random picture of a cat :3
                                (also bot will meow back if you meow at it :3)",
                        )
                        .await;
                }
                "ping" => {
                    // start the timer
                    let start_time = Instant::now();

                    // initial message :3
                    match msg.channel_id.say(&ctx.http, "Pinging.... :3").await {
                        Ok(mut response_msg) => {
                            // calculate the time
                            let new_content = format!(
                                "Pong! Latency is **{}ms** :3",
                                start_time.elapsed().as_millis()
                            );

                            if let Err(err) = response_msg
                                .edit(&ctx.http, EditMessage::new().content(new_content))
                                .await
                            {
                                println!("Error editing the message: {err:?}");
                            }
                        }
                        Err(err) => {
                            println!("Error sending message: {err:?}");
                        }
                    }
                }
                "coinflip" => {
                    let result = if rand::random_bool(1.0 / 2.0) {
                        "Heads :3"
                    } else {
                        "Tails :3"
                    };
                    let _ = msg.channel_id.say(&ctx.http, result).await;
                }
                "hackclub" => {
                    msg.channel_id
                        .say(&ctx.http, "Wish me luck :3 ~ Energy out")
                        .await
                        .expect("Didn't send the message, no hackclub?");
                }
                "cat" => {
                    let randlink = format!("https://cataas.com/cat?a={}", rand::random::<u64>());
                    msg.channel_id
                        .say(&ctx.http, randlink)
                        .await
                        .expect("didnt send cat message");
                }
                _ => {}
            }
        } else {
            // automeower :3
            // meow list :p
            let meows = vec!["meow", "mrow", "nya", "mrrrp", "prrr"];

            // we love hardcoding our stuff
            if msg.author.id != 1527332908287656036 {
                // the thing that checks if message is meowing :3
                if meows.iter().any(|e| msg.content.contains(e)) {
                    let _ = msg.channel_id.say(&ctx.http, "meow:3c").await;
                }
            }
        }

        // async fn ready(&self, _: Context, ready: Ready) {
        //     println!("{} is connected :3", ready.user.name)
        // }
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
    if let Err(err) = client.start().await {
        println!("client error: {err:?}");
    }
}
