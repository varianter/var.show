use azure_functions::{
    bindings::{HttpRequest, HttpResponse, Table},
    func,
    http::Status,
};
use std::env::var;

use super::entities::RedirectEntity;
use super::table::add_redirect_entity_random_key;

#[func]
#[binding(name = "req", methods = "post", route = "random")]
#[binding(name = "output1", table_name = "redirect")]
pub fn random(req: HttpRequest) -> (HttpResponse, Option<Table>) {
    let base_url = var("BaseUrl").unwrap();

    if let Ok(entity) = RedirectEntity::from_request(&req) {
        let (key, table) = add_redirect_entity_random_key(entity);

        return (
            (HttpResponse::build()
                .status(Status::Ok)
                .body(format!("{}/{}", base_url, key))
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
