use serenity::prelude::{Context};
use serenity::model::prelude::{Message};
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};


#[command]
pub fn mh(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut url = "https://monsterhunterworld.wiki.fextralife.com/".to_string();
    let mut h = String::new();

    for i in 0..args.len(){
        if i == 0 {
            h = args.single::<String>().unwrap();
        }
        else{
            let item = args.single::<String>().unwrap();
            h = format!("{}+{}",h, item);
        }
        
    }

    url = format!("{}{}",url, h);

    let resp = reqwest::blocking::get(&url)?;
    if resp.status().is_success() {
        url = format!("༼ つ ◕_◕ ༽つ {}",url);
        let _ = msg.channel_id.say(&ctx.http, url);
    } else {
        let _ = msg.channel_id.say(&ctx.http, "Deine Seite existiert nicht, hier ist eine Katze ฅ^•ﻌ•^ฅ");
    }
    
    Ok(())
}
