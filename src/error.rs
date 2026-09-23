#[derive(thiserror::Error, Debug)]
pub enum AppError{


    #[error("GEMINI_API_KEY is MISSING, Copy env.example and make .env and put the key")]
    MissingApiKey,

    #[error("네트워크 요청 실패 (연결 또는 타임아웃): {0}")]
    Http(#[from] reqwest::Error),

    #[error("Gemini API Error {status}: {message}")]
    Api {status: u16, message:String},

    #[error("JSON cannot be translated: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Gemini answered without text(finish reason:{0})")]
    EmptyResponse(String),

    #[error("Can't read the input : {0}")]
    Io(#[from] std::io::Error),
}