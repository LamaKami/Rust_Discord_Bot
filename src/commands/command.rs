use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
fn commands(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Meinen cuten Commands sind: !pog !add !multi !quickmaths")?;

    Ok(())
}
