use poise::serenity_prelude::GuildId;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

/// Type alias for a hash set of command names
pub type CommandSet = HashSet<String>;

pub struct CommandManager {
    global_toggle: RwLock<CommandSet>,
    guild_toggle: RwLock<HashMap<GuildId, CommandSet>>,
}

/// Manages commands globally and per server. Each CommandSet holds the names of **disabled** commands.
impl CommandManager {
    pub fn new() -> Self {
        CommandManager {
            global_toggle: RwLock::new(CommandSet::new()),
            guild_toggle: RwLock::new(HashMap::new()),
        }
    }

    pub async fn is_enabled(&self, command: &str, guild_id: Option<GuildId>) -> bool {
        let global = self.global_toggle.read().await;
        if global.contains(command) {
            return false;
        }

        if let Some(guild_id) = guild_id {
            let guild_map = self.guild_toggle.read().await;
            if let Some(enabled) = guild_map.get(&guild_id) {
                return !enabled.contains(command);
            }
        }

        true
    }

    /// True if the command was enable or disabled, false if it was already in the desired state.
    pub async fn set_global(&self, command: &str, enabled: bool) -> bool {
        let mut global = self.global_toggle.write().await;
        if enabled {
            global.remove(command)
        } else {
            global.insert(command.to_string())
        }
    }

    /// True if the command was enable or disabled, false if it was already in the desired state.
    pub async fn set_guild(&self, command: &str, guild_id: GuildId, enabled: bool) -> bool {
        let mut guild_map = self.guild_toggle.write().await;
        if guild_map.contains_key(&guild_id) {
            guild_map.insert(guild_id, CommandSet::new());
        }

        let guild = guild_map.get_mut(&guild_id).unwrap();
        if enabled {
            guild.remove(command)
        } else {
            guild.insert(command.to_string())
        }
    }
}
