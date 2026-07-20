use serenity::all::Context;
use serenity::all::Message;
use serenity::builder::EditMessage;
use std::time::Instant;

// ping command to calculate latency :3
pub async fn ping(ctx: &Context, msg: &Message) {
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
