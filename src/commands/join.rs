use serenity::{
    framework::standard::{
        Args, 
        CommandResult,
        macros::command,
    },
    prelude::{
        Context,
        Mentionable,
    },
    model::channel::Message,
};
use lib::msg::check_msg;
use VoiceManager;

#[command]
#[aliases("summon")]
pub fn join(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));

            return Ok(());
        }
    };

    let guild_id = guild.read().id;

    let channel_id = guild
        .read()
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);


    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));

            return Ok(());
        }
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    manager.join(guild_id, connect_to);

    check_msg(msg.channel_id.say(&ctx.http, &format!("Joined {}", connect_to.mention())));

    Ok(())
}