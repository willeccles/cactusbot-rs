use crate::ShardManagerContainer;
use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};

#[command]
#[owners_only]
async fn kys(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "Shutting down <a:kms:805813040327032905>").await?;
        manager.lock().await.shutdown_all().await;
        println!("Shut down by !kys");
    } else {
        msg.reply(ctx, "There was a problem getting the shard manager").await?;

        return Ok(());
    }

    Ok(())
}
