use poise::serenity_prelude::GuildId;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

/// Type alias for a hash set of command names
pub type CommandSet = HashSet<String>;

pub struct CommandManager {
    global_disabled: RwLock<CommandSet>,
    guild_disabled: RwLock<HashMap<GuildId, CommandSet>>,
}

/// Manages commands globally and per server. Each CommandSet holds the names of **disabled** commands.
impl CommandManager {
    pub async fn is_enabled(&self, command: &String, guild_id: Option<GuildId>) -> bool {
        if self.global_disabled.read().await.contains(command) {
            return false;
        }

        let Some(guild_id) = guild_id else {
            return true;
        };

        let guild_map = self.guild_disabled.read().await;

        !guild_map
            .get(&guild_id)
            .is_some_and(|set| set.contains(command))
    }

    /// True if the command was enable or disabled, false if it was already in the desired state.
    pub async fn set_global(&self, command: String, enabled: bool) -> bool {
        let mut global = self.global_disabled.write().await;
        if enabled {
            global.remove(&command)
        } else {
            global.insert(command)
        }
    }

    /// True if the command was enable or disabled, false if it was already in the desired state.
    pub async fn set_guild(&self, command: String, guild_id: GuildId, enabled: bool) -> bool {
        let mut guild_map = self.guild_disabled.write().await;
        let guild = guild_map.entry(guild_id).or_default();

        if enabled {
            guild.remove(&command)
        } else {
            guild.insert(command)
        }
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self {
            global_disabled: RwLock::new(CommandSet::new()),
            guild_disabled: RwLock::new(HashMap::new()),
        }
    }
}
