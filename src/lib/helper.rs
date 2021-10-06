use serenity::{
    client::Context, 
    model::{
        id::{
            ChannelId,
            GuildId,
        }
    }
};

use songbird::{error::JoinResult};
use tracing::{error};

pub async fn join_channel(ctx: &Context, channel_id: ChannelId, guild_id: GuildId) -> JoinResult<()> {
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let handler_lock = manager.get_or_insert(songbird::id::GuildId(guild_id.0));
    let mut handler = handler_lock.lock().await;

    let songbird_channel_id = songbird::id::ChannelId(channel_id.0);
    if handler.current_channel().is_none() || handler.current_channel().unwrap() != songbird_channel_id {
        let handler_res = match handler.join(songbird_channel_id).await {
            Ok(res) => res,
            Err(err) => {
                error!("Failed to send connect request for channel with id {} with err {}", channel_id, err);
                return Err(err);
            }
        };
        drop(handler);
        let _ = match handler_res.await {
            Ok(_res) => _res,
            Err(err) => {
                error!("Failed to connect to channel with id {} with err {}", channel_id, err);
                return Err(err);
            }
        };
    }
    
    return Ok(());
}

pub async fn disconnect_channel(ctx: &Context, guild_id: GuildId) -> JoinResult<()> {
    let manager = songbird::get(&ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let handler_lock = manager.get(guild_id);

    if handler_lock.is_some() {
        let handler_tmp = handler_lock.unwrap();
        let mut handler = handler_tmp.lock().await;

        let self_channel_id = handler.current_channel();

        if self_channel_id.is_some() {                        
            let _ = match handler.leave().await {
                Ok(_res) => _res,
                Err(err) => {
                    error!("Failed to leave voice channel");
                    return Err(err);
                }
            };
        }
    }

    return Ok(());
}