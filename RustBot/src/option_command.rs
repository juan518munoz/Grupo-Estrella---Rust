use crate::{mplayer, CommandResult};
use serenity::client::Context;
use serenity::model::channel::Message;
use std::env;

use crate::list;

pub fn get_command(msg: &Message) -> String {
    let character_discord = env::var("CHARACTER_BOT").expect("Character not found");

    // Remove first character of msg
    let mut input = String::from(&msg.content);
    let prefix = input.remove(0);
    if !character_discord.contains(prefix) {
        return String::from("");
    }

    // Get first word of input
    let mut split_input = input.split_whitespace();

    // instruction is the first word in the message
    let instruction = split_input.next().unwrap();

    // return command
    String::from(instruction)
}

pub async fn run_command(
    command: &str,
    msg: &Message,
    ctx: &Context,
) -> CommandResult {
    match command {
        "help" => Ok(mplayer::help(ctx, msg).await),
        "play" => {
            let result = Ok(mplayer::play(ctx, msg).await);
            return result;
        }
        "pause" => Ok(mplayer::pause(ctx, msg).await),
        "resume" => Ok(mplayer::resume(ctx, msg).await),
        "skip" => Ok(mplayer::skip(ctx, msg).await),
        "leave" => Ok(mplayer::leave(ctx, msg).await),
        "list" => Ok(mplayer::list_ranking(ctx, msg).await),
        _ => Ok({}),
    }
}
