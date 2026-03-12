
#[derive(thiserror::Error, Debug)]
pub enum HenryError {
    #[error("missing environment variable: {0}")]
    MissingEnvironmentVariable(String),
    #[error("invalid environment variable {0}: with value {1}")]
    InvalidEnvironmentVariable(String, String),
    #[error("serenity error: {0}")]
    SerenityError(#[from] poise::serenity_prelude::Error)
}

pub type HenryResult<T> = Result<T, HenryError>;
