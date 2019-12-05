use super::entities::SlackCommand;
use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
};
use std::env::var;

#[func]
pub fn slack(req: HttpRequest) -> HttpResponse {
    let slack_payload = SlackCommand::from_request(&req).unwrap();
    let token = var("SlackCommandToken").unwrap();
    if token == slack_payload.token {}
    return "Hello from Rust!".into();
}
