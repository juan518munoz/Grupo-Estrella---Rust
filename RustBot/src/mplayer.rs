use serenity::model::prelude::Guild;
use std::{env, fs};
use std::io::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use songbird::input::ytdl_search;
use songbird::SerenityInit;

// Usado para el timer
use tokio::time::Duration;
//use tokio::time::sleep;
use songbird::tracks::TrackQueue;

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

// para funcion get_handler - borrar si en la funcion no se mencionan
use songbird::Call;
use std::sync::Arc;

use crate::list;

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

pub async fn help(ctx: &Context, msg: &Message) {
    let _ = msg.channel_id.say(&ctx.http, "Ayuda!").await;
}

pub async fn default_message(ctx: &Context, msg: &Message) {
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            "Ese mensaje no lo entiendo, puedes ver la lista de comandos con `help`",
        )
        .await;
}

// Reproduce cancion, si ya existe una reproduccion en curso, se agrega la cancion solicitada a la cola del reproductor
pub async fn play(ctx: &Context, msg: &Message){
    join(&ctx, &msg).await;
    let handler_lock = match get_manager(&ctx, &msg).await {
        Some(it) => it,
        _ => return,
    };
    let mut handler = handler_lock.lock().await;

    let song = get_song(&ctx, &msg).await;
    
    ///////////////////////////////////////////////////////////////
    let mut file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open("src/song_ranking.txt")
      .unwrap();

    write!(file, "{}\n", song).unwrap();
    //////////////////////////////////////////////////////////////

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

}

// Pausa el reproductor de musica
pub async fn pause(ctx: &Context, msg: &Message) {
    if let Some(handler_lock) = get_manager(&ctx, &msg).await {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.pause();
        let message = String::from("Paused all songs");
        check_msg(msg.channel_id.say(&ctx.http, &message).await);
    }
}

// Resume el reproductor de musica
pub async fn resume(ctx: &Context, msg: &Message) {
    if let Some(handler_lock) = get_manager(&ctx, &msg).await {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.resume();
        let message = String::from("Resumed all songs");
        check_msg(msg.channel_id.say(&ctx.http, &message).await);
    }
}

// Saltea la cancion actual en el reproductor de musica
pub async fn skip(ctx: &Context, msg: &Message) {
    if let Some(handler_lock) = get_manager(&ctx, &msg).await {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.skip();
        let message = String::from("Skipped current song");
        check_msg(msg.channel_id.say(&ctx.http, &message).await);
    }
}

// Quita al bot del canal de voz
pub async fn leave(ctx: &Context, msg: &Message) {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            // El usuario no esta en un canal de voz
            check_msg(msg.reply(ctx, "No estas conectado a un canal").await);
            // Agregar verficacion que el bot este en un canal de voz
            return;
        }
    };

    clear_queue(&ctx, &msg).await;
    if let Some(handler_lock) = get_manager(&ctx, &msg).await {
        let mut handler = handler_lock.lock().await;
        handler.leave().await;
    }
}

// Detiene y elemina la reproduccion de todas las canciones en el reproductor de musica
async fn clear_queue(ctx: &Context, msg: &Message) {
    if let Some(handler_lock) = get_manager(&ctx, &msg).await {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        // iterate over the queue and clear it
        let queue_len = queue.len();
        for _ in 0..queue_len {
            queue.skip();
        }
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
async fn get_song(ctx: &Context, msg: &Message) -> String {
    let character_discord = env::var("CHARACTER_BOT").expect("Character not found");
    let input = String::from(&msg.content);
    let command_play = format!("{}{}", character_discord, "play ");
    let song = input.replace(&command_play, "");
    String::from(song)
}

// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

// list ranking of songs
pub async fn list_ranking(ctx: &Context, msg: &Message) {
    let mut songs_list = list::List::Nil;

    let filename = "src/song_ranking.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, song_name) in reader.lines().enumerate() {
        let song_name = song_name.unwrap(); // Ignore errors.
        // add song to list
        list::add_to_list(&mut songs_list, song_name);
    }

    let mut message = String::from("Lista de canciones:\n");
    
    // ordenamos lista
    let sorted_song_list = list::sort_list(songs_list);

    // pasamos lista a vector
    let songs_vec = list::list_to_vec(&sorted_song_list);

    // iterate vector and add each tuple to message
    for (_index, tupla) in songs_vec.iter().enumerate() {
        let song_name = tupla.0.clone();
        let song_rank = tupla.1;
        let song_rank_str = format!("{}, played: {} time(s)\n", song_name, song_rank);
        message.push_str(&song_rank_str);	
    }

    // enviamos mensaje con string
    check_msg(msg.channel_id.say(&ctx.http, &message).await);

}

