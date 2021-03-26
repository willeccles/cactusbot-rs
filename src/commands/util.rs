use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        Args,
        macros::*,
    },
};

#[command]
#[description = "Show information about this bot."]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let coloropt = match msg.guild_id {
            Some(gid) => match ctx.cache.member(gid, ctx.cache.current_user_id().await).await {
                Some(member) => member.colour(&ctx.cache).await,
                None => None
            }
            None => None
    };

    msg.channel_id.send_message(&ctx.http, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name("Will Eccles");
                author.url("https://eccles.dev/");
                author.icon_url("https://eccles.dev/imgs/avatar.jpg");

                author
            });

            embed.title("Repo: willeccles/cactusbot-rs");
            embed.description("");
            embed.url("https://github.com/willeccles/cactusbot-rs");

            if let Some(color) = coloropt {
                embed.color(color);
            }

            embed
        });

        message
    }).await?;

    Ok(())
}
