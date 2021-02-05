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

fn main() {
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"))
        .expect("Login failed!");

    let (mut connection, _) = discord.connect().expect("Connect failed");
    println!("Ready!");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                match message.content.split_ascii_whitespace().next() {
                    Some("!hello") => {
                        let _ = cmdreply(&discord, message.channel_id,
                                         &("hello, ".to_owned() + &message.author.name));
                    }
                    Some("!pog") => {
                        let _ = cmdreply(&discord, message.channel_id,
                                         "<a:kannapogging:753429010184142968>");
                    }
                    Some("!kys") => {
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
                    Some(&_) => {}
                    None => {}
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
