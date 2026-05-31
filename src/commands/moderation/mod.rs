use crate::{HenryContext, HenryResult};

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("filter"),
    required_permissions = "ADMINISTRATOR",
    guild_only
)]
pub async fn settings(ctx: HenryContext<'_>) -> HenryResult<()> {
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "ADMINISTRATOR",
    guild_only
)]
pub async fn logging(ctx: HenryContext<'_>) -> HenryResult<()> {
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    //subcommands("e"),
)]
pub async fn filter(ctx: HenryContext<'_>) -> HenryResult<()> {
    Ok(())
}
