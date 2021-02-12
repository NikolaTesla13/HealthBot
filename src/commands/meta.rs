use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.channel_id.say(&ctx.http, "pong").await?;

    Ok(())
}

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "world").await?;

    Ok(())
}