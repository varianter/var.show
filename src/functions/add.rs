use super::entities::RedirectEntity;
use azure_functions::{
    bindings::{HttpRequest, HttpResponse, Table},
    func,
    http::Status,
};
use serde_json::Value;

#[func]
#[binding(name = "req", methods = "post", route = "add/{key}")]
#[binding(name = "output1", table_name = "redirect")]
pub fn add(req: HttpRequest) -> (HttpResponse, Option<Table>) {
    if let Ok(entity) = RedirectEntity::from_request(&req) {
        let key = req.route_params().get("key").unwrap();
        let mut table = Table::new();
        {
            let row = table.add_row("with_key", key);
            row.insert(
                "redirect_url".to_string(),
                Value::String(entity.redirect_url),
            );
        }
        return (
            (HttpResponse::build()
                .status(Status::Ok)
                .body(format!("Your key: {}", key))
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
