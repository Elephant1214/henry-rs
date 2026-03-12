use crate::HenryContext;
use poise::serenity_prelude::{
    Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Timestamp,
};

pub struct HenryEmbed {
    embed: CreateEmbed,
}

impl HenryEmbed {
    pub fn basic(
        title: impl Into<String>,
        description: impl Into<String>,
        color: impl Into<Colour>,
    ) -> Self {
        HenryEmbed {
            embed: CreateEmbed::default()
                .title(title)
                .description(description)
                .color(color),
        }
    }

    pub fn with_author(self, ctx: HenryContext<'_>) -> Self {
        HenryEmbed {
            embed: self.embed.author(
                CreateEmbedAuthor::new(ctx.author().tag()).icon_url(
                    ctx.author()
                        .avatar_url()
                        .unwrap_or_else(|| ctx.author().default_avatar_url()),
                ),
            ),
        }
    }

    pub fn add_field(self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        HenryEmbed {
            embed: self.embed.field(name, value, inline),
        }
    }

    pub fn with_footer(self, text: impl Into<String>, icon_url: Option<impl Into<String>>) -> Self {
        let mut footer = CreateEmbedFooter::new(text);
        if icon_url.is_some() {
            footer = footer.icon_url(icon_url.unwrap());
        }

        HenryEmbed {
            embed: self.embed.footer(footer),
        }
    }

    pub fn with_timestamp(self) -> Self {
        HenryEmbed {
            embed: self.embed.timestamp(Timestamp::now()),
        }
    }

    pub fn result(self) -> CreateEmbed {
        self.embed
    }
}
