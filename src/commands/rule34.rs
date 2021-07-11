use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn rule34(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    if msg.channel_id.to_channel(&ctx.http).await?.is_nsfw() {
        match args.single::<String>().unwrap().as_str() {
            "-hentai" => {
    
                match args.trimmed().current() {
                    Some(x) => {
                        //return
                        let url = format!("https://rule34.xxx/index.php?page=dapi&s=post&q=index&json=1&tags={}",x);
    
                        #[derive(serde::Deserialize, Debug)]
                        struct Element {
                            preview_url: String,
                            sample_url: String,
                            file_url: String,
                            directory: u32,
                            hash: String,
                            height: u32,
                            id: u32,
                            image: String,
                            change: u32,
                            owner: String,
                            parent_id: u32,
                            rating: String,
                            sample: u32,
                            sample_height: u32,
                            sample_width: u32,
                            score: u32,
                            tags: String,
                            width: u32,
                        }
    
                        let resp: Vec<Element> = reqwest::get(url)
                            .await?
                            .json()
                            .await?;
    
                        let _msg = msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                //Embed block
                                m.embed(|e| {
                                    e.title("Hentaiesque");
                                    e.description("Une image par Rule34:");
                                    e.image(&resp.first().unwrap().file_url);
                                    e.footer(|f| {
                                        f.text("Rule34 be like");
    
                                        f
                                    });
    
                                    e
                                });
    
                                m
                            }).await;
                    }
                    None => {
                        msg
                            .channel_id
                            .say(&ctx.http, "Veuillez mettre au moins un tag après \"-hentai\", merci")
                            .await?;
                    }
                }
            }
            _ => {
                msg
                    .channel_id
                    .say(&ctx.http, "A po X,(")
                    .await?;
            }
        }
    } else {
        msg
            .channel_id
            .say(&ctx.http, "C po très NSFW tout ça")
            .await?;
    }

    Ok(())
}