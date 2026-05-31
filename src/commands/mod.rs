use crate::{HenryContext, HenryResult};

pub mod admin;
pub mod command_manager;
pub mod miscellaneous;
pub mod moderation;


async fn check_command_enabled(ctx: HenryContext<'_>) -> HenryResult<bool> {
    let name = &ctx.command().name;
    let guild = ctx.guild_id();
    let command_manager = &ctx.data().command_manager;

    let enabled = command_manager.is_enabled(name, guild).await;

    Ok(enabled)
}

fn does_command_exist(ctx: HenryContext<'_>, command: &String) -> bool {
    ctx.data().command_list.contains(command)
}
