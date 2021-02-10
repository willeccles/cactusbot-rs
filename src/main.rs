extern crate discord;

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
