use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub fn mh(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut url = "https://monsterhunterworld.wiki.fextralife.com/".to_string();
    let mut h = "".to_string();

    for i in 0..args.len(){
        if {i == 0}{
            h = args.single::<String>().unwrap();
        }
        else{
            let item = args.single::<String>().unwrap();
            h = format!("{}+{}",h, item);
        }
        
    }
    url = format!("{}{}",url, h);
    
    let _ = msg.channel_id.say(&ctx.http, url);

    Ok(())
}
