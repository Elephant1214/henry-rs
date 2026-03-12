mod commands;
mod db;
mod events;
mod henry_error;
mod embeds;

use crate::commands::command_manager::CommandManager;
use crate::db::HenryDb;
use crate::events::HenryEventHandler;
use crate::henry_error::{HenryError, HenryResult};
use dotenvy::dotenv;
use log::{error, warn};
use poise::builtins::register_globally;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents, UserId};
use poise::{EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions};
use std::collections::HashSet;
use std::default::Default;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

pub struct HenryData {
    pub command_list: Vec<String>,
    pub command_manager: CommandManager,
    pub db: HenryDb,
}

pub type HenryCmdError = Box<dyn Error + Send + Sync>;
pub type HenryContext<'a> = poise::Context<'a, HenryData, HenryCmdError>;

fn read_var(key: &str) -> HenryResult<String> {
    let result = std::env::var(key);
    if result.as_ref().is_ok_and(|string| !string.is_empty()) {
        Ok(result.unwrap())
    } else {
        Err(HenryError::MissingEnvironmentVariable(key.to_string()))
    }
}

fn read_owners() -> HenryResult<HashSet<UserId>> {
    Ok(read_var("OWNERS")?
        .replace(" ", "")
        .split(",")
        .filter_map(|id| match UserId::from_str(id) {
            Ok(user_id) => Some(user_id),
            Err(_) => {
                error!("Invalid Discord user ID found in owner list: {}", id);
                None
            }
        })
        .collect::<HashSet<UserId>>())
}

fn get_framework_options(owners: HashSet<UserId>) -> FrameworkOptions<HenryData, HenryCmdError> {
    FrameworkOptions {
        commands: vec![commands::miscellaneous::ping(), commands::management::settings()],
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

fn build_framework(
    owners: HashSet<UserId>,
    db_path: String,
) -> Framework<HenryData, HenryCmdError> {
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
                        .map(|cmd| cmd.name.clone())
                        .collect(),
                    command_manager: CommandManager::new(),
                    db: HenryDb::new(db_path).await,
                })
            })
        })
        .build()
}

async fn start_bot(
    token: String,
    framework: Framework<HenryData, HenryCmdError>,
) -> HenryResult<()> {
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
        warn!("Not .env file was found: {e}");
    }

    let bot_token = read_var("BOT_TOKEN")?;
    let owners = read_owners()?;

    //let db_url = read_var("DATABASE_URL")?;
    //let db_path = PathBuf::from_str(&db_url)?;

    let framework = build_framework(owners, String::from_str(":memory:").unwrap());
    start_bot(bot_token, framework).await?;

    Ok(())
}
