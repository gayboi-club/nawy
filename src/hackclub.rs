use serenity::all::Context;
use serenity::all::Message;

// silly reusable hackclub command to leave a message for an image :p
pub async fn hackclub(ctx: &Context, msg: &Message) {
    let _ = msg
        .channel_id
        .say(&ctx.http, "Wish me luck :3 ~ Energy out")
        .await;
}
