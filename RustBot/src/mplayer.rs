use std::env;

// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use songbird::input::ytdl_search;
use songbird::SerenityInit;

// Import the `Context` to handle commands.
use serenity::client::Context;

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Result as SerenityResult,
};

struct Handler;

// para funcion get_handler - borrar si en la funcion no se mencionan
use std::sync::Arc;
use songbird::Call;

// !!!!
// https://docs.rs/songbird/latest/songbird/driver/struct.Driver.html#
// https://docs.rs/songbird/0.2.0/songbird/tracks/struct.Track.html#structfield.handle
// !!!!

// join voice channel
pub async fn join(ctx: &Context, msg: &Message) {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            // El usuario no esta en un canal de voz
            check_msg(msg.reply(ctx, "Metete a un canal de voz").await);
            return;
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;
}

// Reproduce cancion, si ya existe una reproduccion en curso, se agrega la cancion solicitada a la cola del reproductor
pub async fn play(ctx: &Context, msg: &Message) {
    join(&ctx, &msg).await;
    
    if let Some(handler_lock) = get_manager(&ctx, &msg).await {
        let mut handler = handler_lock.lock().await;

        let song = get_song(&ctx, &msg).await;

        let source = match songbird::input::ytdl_search(&song).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing song").await);

                return;
            }
        };
        handler.enqueue_source(source);


        let output = String::from("Se agrego: '") + &song + "' a la cola";
        check_msg(msg.channel_id.say(&ctx.http, &output).await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }
}

// Pausa el reproductor de musica
pub async fn pause(ctx: &Context, msg: &Message) {
    if let Some(handler_lock) = get_manager(&ctx, &msg).await {
        let mut handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.pause();
        
        let message = String::from("Stopped all songs");
        check_msg(msg.channel_id.say(&ctx.http, &message).await);
    }
}

// Resume el reproductor de musica
pub async fn resume(ctx: &Context, msg: &Message) {
    if let Some(handler_lock) = get_manager(&ctx, &msg).await {
        let mut handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.resume();
        
        let message = String::from("Resumed all songs");
        check_msg(msg.channel_id.say(&ctx.http, &message).await);
    }

}

// Saltea la cancion actual en el reproductor de musica
pub async fn skip(ctx: &Context, msg: &Message) {
    if let Some(handler_lock) = get_manager(&ctx, &msg).await {
        let mut handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.skip();
        
        let message = String::from("Skipped current song");
        check_msg(msg.channel_id.say(&ctx.http, &message).await);
    }
}

// Devuelve la llamada asociada al servidor
async fn get_manager(ctx: &Context, msg: &Message) -> Option<Arc<Mutex<Call>>> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    
    manager.get(guild_id)
}

// Devuelve el string que contiene la cancion a ser buscada
async fn get_song(_ctx: &Context, msg: &Message) -> String {
    let mut input = String::from(&msg.content);
    input.remove(0); // remove prefix
    let song = input.replace("play ", "");
    String::from(song)
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
