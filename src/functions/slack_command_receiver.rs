use azure_functions::{
    bindings::{HttpRequest, HttpResponse, QueueMessage},
    func,
    http::Status,
};
use serde_json::json;

#[func]
#[binding(name = "req", methods = "post", route = "slack")]
#[binding(name = "output1", queue_name = "slackcommand")]
pub fn slack_command_receiver(req: HttpRequest) -> (HttpResponse, Option<QueueMessage>) {
    let body = req.body();
    if let Some(body_text) = body.as_str() {
        return (
            HttpResponse::build().status(Status::Ok).body("").finish(),
            Some(json!({ "payload": body_text }).into()),
        );
    }

    (
        HttpResponse::build()
            .status(Status::Ok)
            .body("Unable to parse Slack command payload.")
            .finish(),
        None,
    )
}
