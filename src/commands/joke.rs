use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

//Deserialize
use serde::Deserialize;

#[command]
pub async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    
    //Deserialize structs
    #[derive(Deserialize, Debug)]
    struct Base {
        status: i16,
        response: String,
        error: bool,
        joke: Joke
    }

    #[derive(Deserialize, Debug)]
    struct Joke {
        question: String,
        answer: String,
        id: i64
    }

    //Request to get joke
    let resp = reqwest::get("https://blague.xyz/api/joke/random")
        .await?
        .json::<Base>()
        .await?;

    //Create message
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        //Embed block
        m.embed(|e| {
            e.description("Tu veux une blague ?");
            e.field(&resp.joke.question, &resp.joke.answer, false);
            e.footer(|f| {
                f.text("Joke à chier");

                f
            });

            e
        });
        m
    }).await;

    //Print error
    if let Err(why) = msg {
        eprintln!("Error sending message: {:?}", why);
    }

    Ok(())
}