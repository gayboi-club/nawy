use serenity::all::Context;
use serenity::all::Message;

pub async fn cat(ctx: &Context, msg: &Message) {
    let randnum = rand::random::<u64>();
    let randlink = format!("https://cataas.com/cat?a={}", randnum);
    let _ = msg.channel_id.say(&ctx.http, randlink).await;
}
