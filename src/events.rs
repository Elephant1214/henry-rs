use poise::async_trait;
use poise::serenity_prelude::{ActivityData, Context, EventHandler, GuildId, Message};

pub struct HenryEventHandler {}

#[async_trait]
impl EventHandler for HenryEventHandler {
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        ctx.shard.set_activity(Some(ActivityData::custom(format!(
            "Watching {} server{}",
            guilds.len(),
            if guilds.len() > 1 { "s" } else { "" },
        ))))
    }

    async fn message(&self, ctx: Context, new_message: Message) {

    }
}
