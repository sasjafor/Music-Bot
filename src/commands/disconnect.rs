use serenity::{
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
    prelude::{
        Context,
    },
    model::channel::Message,
};
use crate::lib::{
    helper::disconnect_channel,
    msg::check_msg
};

#[command]
#[aliases("leave")]
pub async fn disconnect(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = match ctx.cache.guild_channel(msg.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported").await);
        
            return Ok(());
        },
    };

    let _ = match disconnect_channel(&ctx, guild_id).await {
        Ok(_res) => _res,
        Err(_err) => {
            check_msg(msg.reply(&ctx, "Not in a voice channel").await);
            return Ok(());
        }
    };



    check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);

    Ok(())
}
