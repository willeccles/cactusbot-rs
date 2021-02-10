use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn pog(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "<a:kannapogging:753429010184142968>").await?;

    Ok(())
}
