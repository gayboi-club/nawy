use serenity::all::Context;
use serenity::all::Message;
use std::time::SystemTime;

pub async fn time(ctx: &Context, msg: &Message) {
    let timenow = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    let timemsg = format!("The curent time is <t:{}>", timenow);
    let _ = msg.channel_id.say(&ctx.http, timemsg).await;
}
