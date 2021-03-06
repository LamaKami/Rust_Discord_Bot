mod commands;
use commands::{
    math::*,
    pog::*,
    command::*,
    picture::*,
    mh::*,
};

use serenity::client::Client;
use serenity::prelude::{EventHandler};
use serenity::framework::standard::{
    StandardFramework,
    macros::{
        group
    }
};



#[group]
#[commands(multi,pog,add,commands,quickmaths,monkas,git,watch,amthor,mh)]
struct General;

struct Handler;

impl EventHandler for Handler {}

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