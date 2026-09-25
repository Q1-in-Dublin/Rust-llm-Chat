use crate::error::AppError;
use crate::config::Config;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all ="lowercase")]

pub enum Role{
    User,
    Model,
}
//Rust-JSON
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Content{
    pub role: Role,
    pub parts: Vec<Part>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Part{
    pub text: String,
}

#[derive(serde::Serialize)]
struct GenerateRequest<'a> {
    contents: &'a [Content],
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerateResponse{
    #[serde(default)]
    candidates: Vec<Candidate>,
    usage_metadata : Option<Usage>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all= "camelCase")]
pub struct Candidate {
    content: Option<Content>,
    finish_reason: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Usage{
    pub prompt_token_count : u32, //unsigned 32 bit
    pub candidates_token_count : Option<u32>, //can be or not be u32
    pub total_token_count: u32
}

#[derive(Debug)]
pub struct Reply{
    pub text: String,
    pub usage: Option<Usage>,
}

pub fn parse_success(body: &str) -> Result<Reply, AppError>{
    let response : GenerateResponse = serde_json::from_str(body)?;

    let candidate = match response.candidates.into_iter().next(){
        Some(c) => c,
        None => return Err(AppError::EmptyResponse("unknown".to_string())),
    };

    let finish_reason = candidate
        .finish_reason
        .unwrap_or_else(|| "unknown".to_string());

    let content = match candidate.content{
        Some(c) => c,
        None => return Err(AppError::EmptyResponse(finish_reason)),
    };

    let mut text = String::new();
    for part in content.parts{
        text.push_str(&part.text);
    }

    if text.is_empty(){
        return Err(AppError::EmptyResponse(finish_reason))
    }

    Ok(Reply{
        text,
        usage: response.usage_metadata,
    })

}

pub fn parse_error(status: u16, body: &str) -> AppError{
    let message = serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|json| json["error"]["message"].as_str().map(String::from))
        .unwrap_or_else(|| body.to_string());

    AppError::Api{ status, message }
}

//define data
pub struct GeminiClient{
    http: reqwest::Client,
    api_key: String,
    model: String,
}

//strucutre's method detail
impl GeminiClient{
    pub fn new(config: Config) -> Result<Self, AppError>{
        Ok(GeminiClient{
            http: reqwest::Client::new(),
            api_key: config.api_key,
            model: config.model,
    })
}

    pub async fn generate(&self, history: &[Content]) -> Result<Reply, AppError>{
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            self.model
        );
        
        let request_body = GenerateRequest{contents: history};
        let reponse = self
        .http
        .post(&url)
        .header("x-goog-api-key", &self.api_key)
        .json(&request_body)
        .send() //return future task
        .await?; //async 
        
        let status = reponse.status().as_u16();
        let body = reponse.text().await?;

        if status == 200{
            parse_success(&body)
        }else{
            Err(parse_error(status,&body))
        }
        
}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_success_response() {
        let body = std::fs::read_to_string("tests/fixtures/success.json").unwrap();
        let reply = parse_success(&body).unwrap();

        assert_eq!(reply.text, "안녕하세요! 무엇을 도와드릴까요?");
        assert_eq!(reply.usage.unwrap().total_token_count, 17);
    }
    #[test]
    fn parses_multi_part_response() {
        let body = std::fs::read_to_string("tests/fixtures/multi_part.json").unwrap();
        let reply = parse_success(&body).unwrap();

        assert_eq!(reply.text, "안녕하세요! 두 조각으로 나뉜 응답입니다.");
    }

    #[test]
    fn blocked_response_is_empty_response_error() {
        let body = std::fs::read_to_string("tests/fixtures/blocked.json").unwrap();
        let err = parse_success(&body).unwrap_err();

        match err {
            AppError::EmptyResponse(reason) => assert_eq!(reason, "SAFETY"),
            other => panic!("expected EmptyResponse, got {other:?}"),
        }
    }

    #[test]
    fn empty_candidates_is_empty_response_error() {
        let body = std::fs::read_to_string("tests/fixtures/empty.json").unwrap();
        let err = parse_success(&body).unwrap_err();

        match err {
            AppError::EmptyResponse(reason) => assert_eq!(reason, "unknown"),
            other => panic!("expected EmptyResponse, got {other:?}"),
        }
    }
    #[test]
    fn parses_error_body() {
        let body = r#"{"error": {"message": "API key not valid"}}"#;
        let err = parse_error(401, body);

        match err {
            AppError::Api { status, message } => {
                assert_eq!(status, 401);
                assert_eq!(message, "API key not valid");
            }
            other => panic!("expected Api, got {other:?}"),
        }
    }
}