

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    model::{event::ResumedEvent, gateway::Ready, user::User, id::{RoleId, ChannelId, MessageId, GuildId}, channel::{Reaction, ReactionType::*},},
    prelude::*,
};
use std::io::stdin;
use serenity::http;

use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use isahc::{prelude::*, Request};
use std::time::Duration;

use std::{
    env,
};

async fn send_msg(ctx: Context, channel: ChannelId) -> CommandResult {
    let message = channel.say(&ctx.http, "React with 💧 if you want to be notified!").await?;
    message.react(&ctx.http, Unicode(String::from("💧"))).await?;
    Ok(())
}

async fn start_webhook(ctx: Context) -> CommandResult {
	let url = String::from("https://discord.com/api/webhooks/809750490065010699/8bgQGHId-x3beUSh6E4ZPuMZHRuCj_Y2ZPd6BqHgkj0Y-z2i-4fCu5xkXOKwS9APnQmQ");
  let body = "content=Dont%27t+forget+to+Drink+water+<%40%26809716929203535873>!";
	let mut response = Request::post(url)
    .header("Content-type", "application/x-www-form-urlencoded")
    .body(body)?
    .send()?;	

	Ok(())
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);

        let main_channel = ChannelId(809700999769489408);
        //send_msg(ctx, main_channel).await;

		    start_webhook(ctx).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let user: User = reaction.user(&ctx).await.unwrap();
        if reaction.message_id == MessageId(809707742599708672) {
            let guild_id = match reaction.guild_id {
                Some(id) => id,
                None => {
                  eprintln!("reaction sent in a DM");
                  return;
                },
              };
            let mut member = match guild_id.member(&ctx, user.id).await {
                Ok(member) => member,
                Err(err) => {
                  eprintln!("member could not be retrieved: {}", err);
                  return;
                },
              };
              
            let role_to_add = RoleId(809716929203535873);
            if let Err(err) = member.add_role(&ctx, role_to_add).await {
                eprintln!("role could not be added: {}", err);
            }
        }
	}

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let user: User = reaction.user(&ctx).await.unwrap();
        if reaction.message_id == MessageId(809707742599708672) {
            let guild_id = match reaction.guild_id {
                Some(id) => id,
                None => {
                  eprintln!("reaction sent in a DM");
                  return;
                },
              };
            let mut member = match guild_id.member(&ctx, user.id).await {
                Ok(member) => member,
                Err(err) => {
                  eprintln!("member could not be retrieved: {}", err);
                  return;
                },
              };
              
            let role_to_add = RoleId(809716929203535873);
            if let Err(err) = member.remove_role(&ctx, role_to_add).await {
                eprintln!("role could not be added: {}", err);
            }
        }
	}
}