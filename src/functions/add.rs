use azure_functions::{
    bindings::{HttpRequest, HttpResponse, Table},
    func,
    http::Status,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct RedirectEntity {
    redirect_url: String,
}

#[func]
#[binding(name = "req", methods = "post", route = "add/{key}")]
#[binding(name = "output1", table_name = "redirect")]
pub fn add(req: HttpRequest) -> (HttpResponse, Option<Table>) {
    if let Ok(body) = req.body().as_json::<RedirectEntity>() {
        let key = req.route_params().get("key").unwrap();
        let mut table = Table::new();
        {
            let row = table.add_row("with_key", key);
            row.insert("redirect_url".to_string(), Value::String(body.redirect_url));
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
