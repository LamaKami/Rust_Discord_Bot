use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
fn pog(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pogchamp")?;

    Ok(())
}
#[command]
fn git(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://github.com/LamaKami/Rust_Discord_Bot.git")?;

    Ok(())
}