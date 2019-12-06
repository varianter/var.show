use super::entities::{RedirectEntity, SlackCommand};
use super::table::{add_redirect_entity, add_redirect_entity_random_key};
use azure_functions::{
    bindings::{HttpRequest, HttpResponse, Table},
    func,
    http::Status,
};
use regex::Regex;
use std::env::var;

#[func]
#[binding(name = "req", methods = "post", route = "slack")]
#[binding(name = "output1", table_name = "redirect")]
pub fn slack(req: HttpRequest) -> (HttpResponse, Option<Table>) {
    let slack_payload = SlackCommand::from_request(&req).unwrap();
    let token = var("SlackCommandToken").unwrap();
    let base_url = var("BaseUrl").unwrap();

    if token == slack_payload.token {
        let re = Regex::new(r"(/varshow)\s*(?P<url>\S*)\s*(?P<key>\S+)?").unwrap();

        if let Some(command_capture) = re.captures_iter(slack_payload.command.as_str()).nth(0) {
            if let Some(url_capture) = command_capture.name("url") {
                let key: String;
                let table: Table;

                let url = url_capture.as_str();
                let entity = RedirectEntity {
                    redirect_url: String::from(url),
                    creator: Some(slack_payload.user_name),
                };

                if let Some(key_capture) = command_capture.name("key") {
                    key = String::from(key_capture.as_str());
                    table = add_redirect_entity(&key, entity);
                } else {
                    let (rand_key, c_table) = add_redirect_entity_random_key(entity);
                    key = rand_key;
                    table = c_table;
                }

                return (
                    HttpResponse::build()
                        .status(Status::Ok)
                        .body(format!("{}/{}", base_url, key))
                        .finish(),
                    Some(table),
                );
            }
        }
    }

    (
        HttpResponse::build()
            .status(Status::BadRequest)
            .body("This does not seem to be a valid Slack command.")
            .finish(),
        None,
    )
}
