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
use crate::lib::{
    helper::join_channel,
    msg::check_msg
};

#[command]
#[aliases("summon")]
pub async fn join(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = match msg.guild(&ctx).await {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx, "Groups and DMs not supported").await);
            return Ok(());
        }
    };

    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);


    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx.http, "Not in a voice channel").await);
            return Ok(());
        }
    };

    let _ = match join_channel(ctx, connect_to, guild_id).await {
        Ok(_res) => _res,
        Err(_err) => {
            check_msg(msg.channel_id.say(&ctx, &format!("Failed to join {}", connect_to.mention())).await);
            return Ok(());
        }
    };

    check_msg(msg.channel_id.say(&ctx, &format!("Joined {}", connect_to.mention())).await);

    Ok(())
}