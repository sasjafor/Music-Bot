use VoiceManager;
use lib::msg::check_msg;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::prelude::Mentionable;

pub fn join_channel(ctx: &mut Context, msg: &Message) {
    // check_msg(msg.reply(&ctx, "JOINING TEST!!!"));
    // check_msg(msg.channel_id.say(&ctx.http, "START OF JOIN"));

    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));

            return;
        }
    };
    // check_msg(msg.channel_id.say(&ctx.http, "AFTER GUILD"));

    let guild_id = guild.read().id;

    let channel_id = guild
        .read()
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);


    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));

            return;
        }
    };
    // check_msg(msg.channel_id.say(&ctx.http, "AFTER CONNECT TO"));

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    // check_msg(msg.channel_id.say(&ctx.http, "BEFORE MANAGER JOIN"));
    if manager.join(guild_id, connect_to).is_some() {
        check_msg(msg.channel_id.say(&ctx.http, &format!("Joined {}", connect_to.mention())));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Error joining the channel"));
    }
    // check_msg(msg.channel_id.say(&ctx.http, "END OF JOIN"));
}