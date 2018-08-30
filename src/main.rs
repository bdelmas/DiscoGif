extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serenity;

use serde_json::{Value};
use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("/")) // set the bot's prefix to "~"
        .cmd("hello", ping)
        .cmd("gif", gif));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    println!("INFO: message: {:?}", message.content);
    match message.reply("Hello muchachos!") {
        Ok(_) => (),
        Err(e) => println!("ERROR: Err: {:?}", e),
    }
});

command!(gif(_context, message) {
    let url : String = format!("https://api.giphy.com/v1/gifs/random?api_key={}&tag={}", &env::var("GIPHY_TOKEN").expect("token"), &message.content[5..]);

    println!("INFO: message: {:?}, url: {:?}", message.content, url);

    let body = reqwest::get(&url)?
        .text()?;

    let json : Value = serde_json::from_str(&body).unwrap();
    if let Value::String(ref url) = json["data"]["embed_url"] {
        match message.reply(url) {
            Ok(_) => (),
            Err(e) => println!("ERROR: Err: {:?}", e),
        }
    }
});
