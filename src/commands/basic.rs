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
#[aliases(cf)]
#[description = "Flip a coin."]
async fn coinflip(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, if rand::random() {
        "Heads!"
    } else {
        "Tails!"
    }).await?;

    Ok(())
}

#[command]
#[max_args(1)]
#[description = "Roll a die. You can optionally specify the number of sides on the die."]
async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut sides: u128 = 6u128;

    if !args.is_empty() {
        args.quoted().trimmed();
        match args.parse::<u128>() {
            Ok(0) | Err(_) => {
                msg.channel_id.say(&ctx, format!("Invalid argument: {}", args.current().unwrap())).await?;
                return Ok(());
            }
            Ok(val) => { sides = val; }
        }
    }

    msg.channel_id.say(&ctx, format!("{}", rand::random::<u128>() % sides + 1)).await?;

    Ok(())
}
