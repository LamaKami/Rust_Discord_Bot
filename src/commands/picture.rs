use std::{path::Path};
use serenity::{
    model::{channel::Message},
    prelude::*,
    http::AttachmentType,
};
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
//
#[command]
fn monkas(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.add_file(AttachmentType::Path(Path::new("./pictures/monkas.png")));
            m
        });
    Ok(())

}
