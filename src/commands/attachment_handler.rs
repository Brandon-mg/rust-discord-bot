use crate::{commands::image_effects, Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};

/// View the difference between two file sizes
#[poise::command(prefix_command, slash_command)]
pub async fn file_details(
    ctx: Context<'_>,
    #[description = "File to examine"] file: serenity::Attachment,
) -> Result<(), Error> {
    let mut message = String::from("not an image");

    if file.content_type.unwrap().contains("image") {
        message = String::from("is image");
        //ctx.send(builder)
    }
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn totalsize(
    ctx: Context<'_>,
    #[description = "File to rename"] files: Vec<serenity::Attachment>,
) -> Result<(), Error> {
    let total = files.iter().map(|f| f.size as u64).sum::<u64>();

    ctx.say(format!(
        "Total file size: `{}B`. Average size: `{}B`",
        total,
        total.checked_div(files.len() as _).unwrap_or(0)
    ))
    .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn fractal(
    ctx: Context<'_>,
    #[description = "random seed"] seed: Option<u32>,
) -> Result<(), Error> {
    let mut default: u32 = 0;
    if let Some(seed) = seed{
        default+=seed
    }
    let img_buf = image_effects::make_fractal(default);
    let attachment = serenity::CreateAttachment::bytes(img_buf, String::from("image.png"));
    ctx.send(CreateReply::default().content("image").attachment(attachment)).await?;
    Ok(())
}
