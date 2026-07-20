use serenity::all::Context;
use serenity::all::Message;

pub async fn help(ctx: &Context, msg: &Message) {
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            "\
            List of commands :3c
- .ping -> pings server
- .coinflip -> does a coinflip :3
- .cat -> returns random picture of a cat :3
- meow -> bot will meow back :3
- time -> tells the current time :3",
        )
        .await;
}
