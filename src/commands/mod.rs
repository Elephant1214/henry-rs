use crate::HenryContext;
use crate::henry_error::HenryResult;

pub mod command_manager;
pub mod management;
pub mod miscellaneous;

async fn check_command_enabled(ctx: HenryContext<'_>) -> HenryResult<bool> {
    let command = ctx.command().name.clone();
    let guild = ctx.guild_id();
    let command_manager = &ctx.data().command_manager;
    let enabled = command_manager.is_enabled(&*command, guild).await;

    Ok(enabled)
}
