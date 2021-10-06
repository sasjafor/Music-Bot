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
use songbird::{
    create_player, 
    input::{Restartable, restartable, ytdl_search}, 
    ytdl
};
use tracing::{error};
use crate::lib::{
    helper::join_channel,
    msg::check_msg
};

#[command]
#[aliases("p")]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // let url = match args.single::<String>() {
    //     Ok(url) => url,
    //     Err(_) => {
    //         check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

    //         return Ok(());
    //     },
    // };

    // if !url.starts_with("http") {
    //     check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

    //     return Ok(());
    // }
    
    let query = args.rest();

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

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match ytdl_search(&query).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            },
        };

        let (audio, _audio_handle) = create_player(source);

        handler.play(audio);

        check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    Ok(())
}