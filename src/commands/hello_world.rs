use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn hello_world(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Hello world!").await?;

    Ok(())
}