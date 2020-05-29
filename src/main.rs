mod commands;
use std::{path::Path};
use serenity::client::Client;
use serenity::prelude::{EventHandler};
use serenity::framework::standard::{
    StandardFramework,
    macros::{
        group
    }
};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    http::AttachmentType,
};
use commands::{
    math::*,
    pog::*,
    command::*,
    picture::*,
};

#[group]
#[commands(multi,pog,add,commands,quickmaths,monkas)]
struct General;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!monkasss"{
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.add_file(AttachmentType::Path(Path::new("./monkas.png")));
                m
            });
        }
    }
}

fn main() {
    // Login with a bot token from the environment
    let token = include_str!("../.token");

	let mut client =
        Client::new(&token, Handler).expect("Error creating client");
        
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}