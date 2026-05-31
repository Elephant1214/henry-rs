mod commands;
mod db;
mod embeds;
mod events;

use crate::commands::command_manager::CommandManager;
use crate::db::HenryDb;
use crate::events::HenryEventHandler;
use anyhow::{Context, bail};
use dotenvy::dotenv;
use log::{error, warn};
use poise::builtins::register_globally;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents, UserId};
use poise::{EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions};
use std::collections::HashSet;
use std::default::Default;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

pub struct HenryData {
    pub command_list: Vec<String>,
    pub command_manager: CommandManager,
    pub db: HenryDb,
}

pub type HenryResult<T> = anyhow::Result<T>;
pub type HenryError = anyhow::Error;

pub type HenryContext<'a> = poise::Context<'a, HenryData, HenryError>;

fn read_var(key: &str) -> HenryResult<String> {
    let value = std::env::var(key).context(format!("Missing environment variable {}", key))?;

    if value.is_empty() {
        bail!("Environment variable {} is empty", key);
    }

    Ok(value)
}

fn read_owners() -> HenryResult<HashSet<UserId>> {
    Ok(read_var("OWNERS")?
        .split(",")
        .map(str::trim)
        .filter_map(|id| {
            UserId::from_str(id)
                .inspect_err(|_| {
                    error!("Invalid Discord user ID found in owner list: {}", id);
                })
                .ok()
        })
        .collect())
}

fn get_framework_options(owners: HashSet<UserId>) -> FrameworkOptions<HenryData, HenryError> {
    FrameworkOptions {
        commands: vec![commands::miscellaneous::ping(), commands::admin::admin()],
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("h!".into()),
            edit_tracker: Some(Arc::new(EditTracker::for_timespan(Duration::from_secs(
                3600,
            )))),
            ..Default::default()
        },
        owners,
        ..Default::default()
    }
}

fn build_framework(owners: HashSet<UserId>, db_path: String) -> Framework<HenryData, HenryError> {
    let options = get_framework_options(owners);

    Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                register_globally(ctx, &framework.options().commands).await?;

                Ok(HenryData {
                    command_list: framework
                        .options()
                        .commands
                        .iter()
                        .map(|cmd| cmd.name.to_string())
                        .collect(),
                    command_manager: CommandManager::default(),
                    db: HenryDb::new(db_path).await?,
                })
            })
        })
        .build()
}

async fn start_bot(token: String, framework: Framework<HenryData, HenryError>) -> HenryResult<()> {
    let mut client = ClientBuilder::new(token, GatewayIntents::all())
        .framework(framework)
        .event_handler(HenryEventHandler {})
        .await?;

    Ok(client.start().await?)
}

#[tokio::main]
async fn main() -> HenryResult<()> {
    tracing_subscriber::fmt::init();

    if let Err(e) = dotenv() {
        warn!("No .env file was found: {e}");
    }

    let bot_token = read_var("BOT_TOKEN")?;
    let owners = read_owners()?;

    //let db_url = read_var("DATABASE_URL")?;
    //let db_path = PathBuf::from_str(&db_url)?;

    let framework = build_framework(owners, String::from(":memory:"));
    start_bot(bot_token, framework).await?;

    Ok(())
}
