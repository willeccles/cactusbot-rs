mod commands;

use std::{
    collections::HashSet,
    env,
    sync::Arc,
};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, Reason,
        StandardFramework,
        macros::*,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        event::ResumedEvent,
        gateway::{Ready, Activity, ActivityType},
        id::UserId,
        permissions::Permissions,
    },
    prelude::*,
};

use commands::{
    basic::*,
    owner::*,
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);

        let _ = ctx.set_activity(Activity::competing("chess")).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[group]
#[only_in(guilds)]
#[summary = "General-purpose commands"]
#[commands(coinflip, roll)]
struct General;

#[group]
#[owners_only]
#[summary = "Owner-only commands"]
#[commands(kys)]
struct Owner;

#[help]
#[command_not_found_text = "Command not found: `{}`."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
    ) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn unknown_command(ctx: &Context, msg: &Message, cmd: &str) {
    let _ = msg.channel_id.say(&ctx, format!("Command not found: {}", cmd)).await;
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::Ratelimited(info) => {
            if info.is_first_try {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, &format!("Try again in {} seconds.", info.as_secs()))
                    .await;
            }
        }
        DispatchError::OnlyForOwners => {
            let _ = msg.channel_id.say(&ctx, "You're not the boss of me >:(").await;
        }
        DispatchError::TooManyArguments{max, given} => {
            let _ = msg
                .channel_id
                .say(&ctx, format!("Too many arguments (max {}, found {})", max, given))
                .await;
        }
        DispatchError::NotEnoughArguments{min, given} => {
            let _ = msg
                .channel_id
                .say(&ctx, format!("Not enough arguments (min {}, found {})", min, given))
                .await;
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected token");

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                   .on_mention(Some(bot_id))
                   .prefix("$")
                   .owners(owners))
        .unrecognised_command(unknown_command)
        .on_dispatch_error(dispatch_error)
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
