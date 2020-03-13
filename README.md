# Punk Bot (Rust version)
A Discord music bot without limits.

## Usage
To use the bot you need to provide an authorisation token for a Discord application with the `DISCORD_APP_AUTH_TOKEN` environment variable. Additionally, a YouTube API key is needed, which is expected to be stored in `YOUTUBE_API_KEY`.

## Commands
* !p or !play [search string | url] - To search and play a youtube video or play any media that are supported by youtube-dl
* !loop - Enable loop
* !skip - Skip what is currently playing
