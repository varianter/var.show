use azure_functions::{
    bindings::{HttpRequest, HttpResponse, Table},
    func,
    http::Status,
};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::Value;

use super::entities::RedirectEntity;

#[func]
#[binding(name = "req", methods = "post", route = "random")]
#[binding(name = "output1", table_name = "redirect")]
pub fn random(req: HttpRequest) -> (HttpResponse, Option<Table>) {
    if let Ok(entity) = RedirectEntity::from_request(&req) {
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        let mut table = Table::new();
        {
            let row = table.add_row("with_key", rand_string.as_str());
            row.insert(
                "redirect_url".to_string(),
                Value::String(entity.redirect_url),
            );
        }

        return (
            (HttpResponse::build()
                .status(Status::Ok)
                .body(format!("Your key: {}", rand_string))
                .finish()),
            Some(table),
        );
    }

    (
        HttpResponse::build()
            .status(Status::BadRequest)
            .body("Request could not be parsed. Did you include redirect_url?")
            .finish(),
        None,
    )
}
