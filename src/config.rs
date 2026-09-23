use crate::cli::Cli;
use crate::error::AppError;

pub struct Config{
    //container
    pub api_key : String,
    pub model : String,
}

impl Config{
    pub fn load(cli: &Cli) -> Result<Self, AppError>{
        dotenvy::dotenv().ok();

        let api_key = std::env::var("GEMINI_API_KEY")
        .ok()
        .filter(|key| !key.is_empty())
        .ok_or(AppError::MissingApiKey)?;

        let model = cli.model.clone()
        .or_else(|| std::env::var("GEMINI_MODEL").ok())
        .unwrap_or_else(|| "gemini-3.5-flash-lite".to_string());

        Ok(Config {api_key,model})
    }
}