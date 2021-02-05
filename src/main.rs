extern crate discord;

use discord::model::Event;
use discord::Discord;
use std::env;

fn main() {
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"))
        .expect("Login failed!");

    let (mut connection, _) = discord.connect().expect("Connect failed");
    println!("Ready!");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);
                match message.content.split_ascii_whitespace().next() {
                    Some("!hello") => {
                        let _ = discord.send_message(
                            message.channel_id,
                            &("hello, ".to_owned() + &message.author.name),
                            "",
                            false,
                            );
                    }
                    Some("!kys") => {
                        let isme = message.author.id == discord::model::UserId(111943010396229632);
                        let _ = discord.send_message(
                            message.channel_id,
                            if !isme {
                                "You're not the boss of me >:("
                            } else {
                                "<a:kms:805813040327032905>"
                            },
                            "",
                            false,
                            );
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
