use crate::commands::check_command_enabled;
use crate::embeds::HenryEmbed;
use crate::{HenryCmdError, HenryContext};
use poise::serenity_prelude::colours::branding::YELLOW;
use poise::serenity_prelude::colours::roles::{BLUE, GREEN, RED};
use poise::CreateReply;

#[poise::command(slash_command, prefix_command, check = "check_command_enabled")]
pub async fn ping(ctx: HenryContext<'_>) -> Result<(), HenryCmdError> {
    let ping = ctx.ping().await.as_millis();
    let color = if ping == 0 {
        BLUE
    } else if ping < 100 {
        GREEN
    } else if ping < 250 {
        YELLOW
    } else {
        RED
    };

    ctx.send(
        CreateReply::default().ephemeral(true).embed(
            HenryEmbed::basic("Pong!", format!("Gateway latency: {ping}ms"), color)
                .with_author(ctx)
                .with_timestamp()
                .result(),
        ),
    )
    .await?;

    Ok(())
}
