use serenity::all::Context;
use serenity::all::Message;

//coinflip command :3
pub async fn coinflip(ctx: &Context, msg: &Message) {
    let flip = rand::random();
    let result = if flip { "Heads :3" } else { "Tails :3" };
    let _ = msg.channel_id.say(&ctx.http, result).await;
}
