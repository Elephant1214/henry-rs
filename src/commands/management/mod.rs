use crate::HenryContext;
use crate::henry_error::HenryResult;
use poise::serenity_prelude::{Colour, CreateEmbed};
use poise::{ChoiceParameter, CreateReply};

#[poise::command(slash_command, prefix_command, subcommands("enable", "disable"))]
pub async fn settings(ctx: HenryContext<'_>) -> HenryResult<()> {
    Ok(())
}

#[derive(Debug, Clone, Copy, ChoiceParameter)]
pub enum CmdScope {
    Guild,
    Global,
}

fn does_command_exist(ctx: HenryContext<'_>, command: &String) -> bool {
    ctx.data().command_list.contains(command)
}

#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn enable(
    ctx: HenryContext<'_>,
    command: String,
    scope: Option<CmdScope>,
) -> HenryResult<()> {
    let command = command.to_lowercase();

    let reply = if does_command_exist(ctx, &command) {
        CreateReply::default().ephemeral(true).embed(
            CreateEmbed::default()
                .title("Command Settings")
                .description(format!("Enable: {:?}", scope))
                .color(Colour::BLUE),
        )
    } else {
        CreateReply::default().ephemeral(true).embed(
            CreateEmbed::default()
                .title("Command Settings")
                .description(format!("The command `{}` does not exist!", command))
                .color(Colour::BLUE),
        )
    };

    ctx.send(reply).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn disable(
    ctx: HenryContext<'_>,
    command: String,
    scope: Option<CmdScope>,
) -> HenryResult<()> {
    let command = command.to_lowercase();

    let reply = if does_command_exist(ctx, &command) {
        CreateReply::default().ephemeral(true).embed(
            CreateEmbed::default()
                .title("Command Settings")
                .description(format!("Disable: {:?}", scope))
                .color(Colour::BLUE),
        )
    } else {
        CreateReply::default().ephemeral(true).embed(
            CreateEmbed::default()
                .title("Command Settings")
                .description(format!("The command `{}` does not exist!", command))
                .color(Colour::BLUE),
        )
    };

    ctx.send(reply).await?;

    Ok(())
}
