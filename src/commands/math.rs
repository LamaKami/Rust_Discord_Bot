use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub fn multi(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    //Moeglichkeit 1
    let mut product = 1.0;

    for _ in 0..args.len(){
        let number = args.single::<f64>().unwrap();
        product = product * number;
    }

    let _ = msg.channel_id.say(&ctx.http, product);

    Ok(())
}


#[command]
pub fn add(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    //Moeglichkeit 2
    let mut sum = 0.0;

    for arg in args.iter::<f32>() {
        let arg = arg.unwrap();
        sum = sum + arg;
    }

    let _ = msg.channel_id.say(&ctx.http, sum);

    Ok(())
}

#[command]
fn quickmaths(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "2+2 is 4 minus 1 that's 3")?;

    Ok(())
}