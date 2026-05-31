use poise::serenity_prelude::GuildId;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct GuildData {
    pub id: GuildId,
}
