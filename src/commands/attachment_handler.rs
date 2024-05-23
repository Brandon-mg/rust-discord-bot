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

#[poise::command(prefix_command)]
pub async fn fractal(
    ctx: Context<'_>,
    #[description = "random seed"] seed: Option<i32>,
) -> Result<(), Error> {
    let mut imgBuf = image_effects::make_fractal();
    let attachment = serenity::CreateAttachment::bytes(imgBuf, String::from("image.png"));
    ctx.send(CreateReply::default().content("image").attachment(attachment)).await?;

    Ok(())
}
