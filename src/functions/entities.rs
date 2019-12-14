use azure_functions::bindings::{HttpRequest, QueueMessage};
use serde::de::value::Error as DeserializeError;
use serde::Deserialize;
use serde_json::Result as JsonResult;
use serde_urlencoded;

#[derive(Deserialize)]
pub struct RedirectEntity {
    pub redirect_url: String,
    pub creator: Option<String>,
}

impl RedirectEntity {
    pub fn from_request(req: &HttpRequest) -> JsonResult<RedirectEntity> {
        req.body().as_json::<RedirectEntity>()
    }
}

#[derive(Deserialize)]
pub struct SlackCommand {
    pub token: String,
    pub team_id: String,
    pub team_domain: String,
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
}

impl SlackCommand {
    pub fn from_request(req: &HttpRequest) -> std::result::Result<SlackCommand, DeserializeError> {
        let body = req.body();
        let body_text = body.as_str().unwrap();
        SlackCommand::from_str(body_text)
    }

    pub fn from_str(text: &str) -> std::result::Result<SlackCommand, DeserializeError> {
        serde_urlencoded::from_str(text)
    }
}

#[derive(Deserialize)]
pub struct SlackPayload {
    pub payload: String,
}

impl SlackPayload {
    pub fn from_queue_message(message: &QueueMessage) -> JsonResult<SlackPayload> {
        message.as_json::<SlackPayload>()
    }
}
