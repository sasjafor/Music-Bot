#[macro_use]
extern crate log;

extern crate env_logger;
extern crate serenity;

mod lib;
mod commands;

use std::{
    env, 
    sync::Arc,
};

use serenity::{
    client::{
        Context,
        Client,
        EventHandler,
        bridge::voice::ClientVoiceManager,
    },
    framework::{
        standard::{
            macros::{
                group,
            },
        },
        StandardFramework
    },
    model::{
        event::ResumedEvent,
        gateway::Ready,
        id::ChannelId,
        guild::Guild,
    },
    prelude::{
        Mutex,
        TypeMapKey,
    },
};

use commands::{
    join::*,
    play::*,
    disconnect::*,
};

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(join, play, disconnect)]
struct General;

fn main() {
    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_APP_AUTH_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler)
        .expect("Error creating client");

    let _yt_api_key = env::var("YOUTUBE_API_KEY")
        .expect("Expected a youtube api key in the environment");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    client.with_framework(StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .group(&GENERAL_GROUP)
            .unrecognised_command(|_, _, unknown_command_name| {
                println!("Could not find command named '{}'", unknown_command_name)
            })
        ); // set the bot's prefix to "!"

    // start listening for events by starting a single shard
    let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}

fn _voice_channel_is_empty(ctx: &mut Context, guild: Guild, channel_id: ChannelId) -> bool {
    let mut is_empty = true;
    for state in guild.voice_states.values().filter(|state| state.channel_id == Some(channel_id)) {
        let user = match state.user_id.to_user(&ctx) {
            Ok(user) => user,
            Err(err) => {
                error!("Error retrieving user: {:?}", err);
                return is_empty;
            }
        };
        is_empty &= user.bot;
    }
    return is_empty;
}
