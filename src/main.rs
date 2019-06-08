extern crate reqwest;
#[macro_use] extern crate serenity;

use serenity::client::Client;
use serenity::model::channel::Reaction;
use serenity::model::event::TypingStartEvent;
use serenity::model::gateway::Game;
use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;
use std::collections::HashMap;
use std::env;
use std::fmt;

struct Handler;

impl EventHandler for Handler {
    fn typing_start(&self, ctx: Context, _event: TypingStartEvent) {
        ctx.set_game(Game::playing("Waiting on typing..."));
    }

    fn reaction_add(&self, ctx: Context, _add_reaction: Reaction) {
        ctx.set_game(Game::playing("Reacting..."));
    }
}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("luabot "))
        .cmd("dog", dog));

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[derive(Debug)]
struct NoDogError {}

impl fmt::Display for NoDogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing a dog link")
    }
}

impl std::error::Error for NoDogError {}

fn fetch_dog_link() -> Result<String, Box<std::error::Error>> {
    let mut response: HashMap<String, String> =
        reqwest::get("https://dog.ceo/api/breeds/image/random")?.json()?;

    match response.remove("message") {
        Some(link) => Ok(link),
        None => Err(Box::new(NoDogError{}))
    }
}

command!(dog(_context, message) {
  let link = fetch_dog_link()?;
  message.reply(&link)?;
});
