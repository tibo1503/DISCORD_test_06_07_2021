use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::http::AttachmentType;

use std::path::PathBuf;
use std::{fs, io};

use rand::seq::SliceRandom;

#[command]
pub async fn natsuki(ctx: &Context, msg: &Message) -> CommandResult {

    let picture_path: PathBuf;
    {
        let entries = fs::read_dir("./assets/natsuki_image/")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        picture_path = entries.choose(&mut rand::thread_rng()).unwrap().to_owned();
    }

    let mut file_name = format!("{:?}",picture_path)
        .split("/")
        .last()
        .unwrap()
        .to_owned();
    file_name.pop();


    //Create message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            //Embed block
            m.embed(|e| {
                e.title("F..ING NATSUKI !!!");
                e.description(format!("Picture with name {}:", file_name));
                e.image(format!("attachment://{}", file_name));
                e.footer(|f| {
                    f.text("Natsuki be like");

                    f
                });

                e
            });
            m.add_file(AttachmentType::Path(std::path::Path::new(&picture_path)));

            m
        }).await;


    //Print error
    if let Err(why) = msg {
        eprintln!("Error sending message: {:?}", why);
    }
    

    Ok(())
}