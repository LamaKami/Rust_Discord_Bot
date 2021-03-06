use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
fn message(ctx: &mut Context, msg: &Message) -> CommandResult{
    if msg.content == "!hello" {
        // The create message builder allows you to easily create embeds and messages
        // using a builder syntax.
        // This example will create a message that says "Hello, World!", with an embed that has
        // a title, description, three fields, and a footer.
        let msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.content("Hello, World!");
            m.add_file(AttachmentType::Path(Path::new("./pictures/monkas.png")));
            m
        });

        if let Err(why) = msg {
            println!("Error sending message: {:?}", why);
        }
    }
    Ok(())
}
    