use crate::{HenryCmdError, HenryContext};

pub mod miscellaneous;
pub mod command_manager;
pub mod management;

async fn check_command_enabled(ctx: HenryContext<'_>) -> Result<bool, HenryCmdError> {
    let command = ctx.command().name.clone();
    let guild = ctx.guild_id();
    let command_manager = &ctx.data().command_manager;
    let enabled = command_manager.is_enabled(&*command, guild).await;

    Ok(enabled)
}
