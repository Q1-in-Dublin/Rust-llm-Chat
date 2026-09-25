mod error;
//bring file from cli.rs
mod cli;
mod config;
mod gemini;

use error::AppError;
use gemini::{Content, GeminiClient, Part, Role};


#[tokio::main]
async fn main()-> Result<(), AppError>{
    let config = config::Config::load(&cli::Cli { prompt: None, model: None, usage: false })?;
    let client = GeminiClient::new(config)?;

    let history = vec![Content {
        role: Role::User,
        parts: vec![Part { text: "안녕, 넌 누구야?".to_string() }],
    }];

    let reply = client.generate(&history).await?;
    println!("{}", reply.text);

    Ok(())
}

