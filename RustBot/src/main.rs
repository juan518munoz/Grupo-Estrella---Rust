extern crate dotenv;

use dotenv::dotenv;
use mplayer::List;
use std::env;

// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use songbird::SerenityInit;

// Import the `Context` to handle commands.
use serenity::client::Context;

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult, Delimiter,
        },
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Result as SerenityResult,
};

mod mplayer;
#[path = "./option_command.rs"]
pub mod option_command;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut songs_list = mplayer::initialize();
        println!("a{:?}", songs_list);
        let command_option: String = option_command::get_command(&msg);
        option_command::run_command(&command_option, &msg, &ctx, &mut songs_list).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
struct General;

#[tokio::main]
async fn main() {
    // Solicita token del bot al iniciarse
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });
    tokio::signal::ctrl_c().await;
    println!("Chauu.");
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
