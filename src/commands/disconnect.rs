use serenity::{
    framework::standard::{
        CommandResult,
        macros::command,
    },
    prelude::{
        Context,
    },
    model::channel::Message,
};
use lib::msg::check_msg;
use VoiceManager;

#[command]
#[aliases("leave")]
fn disconnect(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id);

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel"));
    } else {
        check_msg(msg.reply(&ctx, "Not in a voice channel"));
    }

    Ok(())
}
