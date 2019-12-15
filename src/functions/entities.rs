use azure_functions::bindings::{HttpRequest, QueueMessage};
use serde::Deserialize;
use serde_json::Result as JsonResult;

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
pub struct SlackPayload {
    pub payload: String,
}

impl SlackPayload {
    pub fn from_queue_message(message: &QueueMessage) -> JsonResult<SlackPayload> {
        message.as_json::<SlackPayload>()
    }
}
