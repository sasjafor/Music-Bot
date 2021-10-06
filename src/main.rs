mod lib;
mod commands;

use std::{
    collections::HashSet,
    env,
    sync::{
        Arc,
    },
};

// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use songbird::SerenityInit;

use serenity::{
    async_trait,
    client::{
        bridge::{
            gateway::{
                GatewayIntents,
                ShardManager,
            }
        },
        Client, 
        Context,
        EventHandler
    },
    framework::{
        standard::{
            Args,
            CommandGroup,
            CommandResult,
            HelpOptions,
            help_commands,
            macros::{
                group,
                help,
            }, 
        },
        StandardFramework
    },
    http::Http,
    model::{
        event::ResumedEvent, 
        gateway::Ready, 
        guild::Guild, 
        id::{
            ChannelId,
            UserId,
        }, 
        prelude::*,
    },
    prelude::{
        TypeMapKey,
        Mutex,
    },
};

use tracing::{error, info};
use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};

use commands::{
    join::*,
    play::*,
    disconnect::*,
};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(join, play, disconnect)]
#[only_in("guilds")]
#[help_available]
struct General;

#[help]
#[no_help_available_text("No help available for this command")]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::plain(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    // Login with a bot token from the environment
    let token = env::var("DISCORD_APP_AUTH_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (_owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let _yt_api_key = env::var("YOUTUBE_API_KEY")
        .expect("Expected a youtube api key in the environment");

    // Create the framework
    let framework = StandardFramework::new()
        .group(&GENERAL_GROUP)
            .help(&MY_HELP)
            .configure(|c| c
                .prefix("!")
                .allow_dm(false)
                .case_insensitivity(true)
            );

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .intents(   GatewayIntents::GUILD_MEMBERS |
                    GatewayIntents::GUILD_MESSAGES |
                    GatewayIntents::GUILD_VOICE_STATES |
                    GatewayIntents::GUILDS)
        .register_songbird()
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

async fn _voice_channel_is_empty(ctx: &Context, guild: &Guild, channel_id: ChannelId) -> bool {
    let mut is_empty = true;
    for state in guild
        .voice_states
        .values()
        .filter(|state| state.channel_id == Some(channel_id))
    {
        let user = match state.user_id.to_user(ctx).await {
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
