use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
fn pog(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pogchamp")?;

    Ok(())
}