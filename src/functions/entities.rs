use azure_functions::bindings::HttpRequest;
use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize)]
pub struct RedirectEntity {
    pub redirect_url: String,
}

impl RedirectEntity {
    pub fn from_request(req: &HttpRequest) -> Result<RedirectEntity> {
        req.body().as_json::<RedirectEntity>()
    }
}
