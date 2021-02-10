mod commands;

use std::{
    collections::HashSet,
    env,
    sync::Arc,
};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
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
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[group]
#[commands(hello, pog, kys)]
struct General;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected token");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                   .prefix("!")
                   .owners(owners))
        .group(&GENERAL_GROUP);

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

/*extern crate discord;

use discord::model::{Event, ChannelId, UserId, Message};
use discord::Discord;
use std::env;

fn cmdreply(
    discord: &Discord,
    channel: ChannelId,
    msg: &str
    ) -> discord::Result<Message> {

    discord.send_message(channel, msg, "", false)
}

fn cmd_hello(author: &str, arg: &str) -> String {
    format!("hello, {}", if arg.len() > 0 { arg } else { author })
}

fn main() {
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"))
        .expect("Login failed!");

    let (mut connection, _) = discord.connect().expect("Connect failed");
    println!("Ready!");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                // epic(?) new command parsing
                if message.content.starts_with('!') {
                    let cmdparts: Vec<&str> = message.content.splitn(2, ' ').collect();
                    match cmdparts.first().unwrap() {
                        &"!hello" => {
                            let _ = cmdreply(&discord, message.channel_id,
                                             &cmd_hello(&message.author.name,
                                                        cmdparts.get(1).unwrap_or(&"")));
                        }
                        &_ => {}
                    }
                }

                // not epic command parsing maybe
                if message.content.starts_with("!pog") {
                    let _ = cmdreply(&discord, message.channel_id,
                                     "<a:kannapogging:753429010184142968>");
                } else if message.content.starts_with("!kys") {
                    let isme = message.author.id == UserId(111943010396229632);
                    let _ = cmdreply(&discord, message.channel_id,
                                     if !isme {
                                         "You're not the boss of me >:("
                                     } else {
                                         "<a:kms:805813040327032905>"
                                     });

                    if isme {
                        break;
                    }
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }
}
*/
