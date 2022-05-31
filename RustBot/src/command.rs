use std::env;
use serenity::model::channel::Message;

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