use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    trace!("ping() called");
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}
