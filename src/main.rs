use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::process::Command;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "pelo amor de Deus para com isso" {
            let output = Command::new("taskkill")
                .arg("/IM")
                .arg("LeagueClient.exe")
                .arg("/F")
                .output()
                .expect("failed to execute process");

            println!("{:?}", output);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = "your_token_here";

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
