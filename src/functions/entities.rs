use azure_functions::bindings::HttpRequest;
use serde::de::value::Error as DeserializeError;
use serde::Deserialize;
use serde_json::Result as JsonResult;
use serde_urlencoded;

#[derive(Deserialize)]
pub struct RedirectEntity {
    pub redirect_url: String,
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
    pub text: u32,
    pub response_url: String,
}

impl SlackCommand {
    pub fn from_request(req: &HttpRequest) -> std::result::Result<SlackCommand, DeserializeError> {
        serde_urlencoded::from_str(req.body().as_str().unwrap())
    }
}
