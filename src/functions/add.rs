use super::entities::RedirectEntity;
use super::table::add_redirect_entity;
use azure_functions::{
    bindings::{HttpRequest, HttpResponse, Table},
    func,
    http::Status,
};
use std::env::var;

#[func]
#[binding(name = "req", methods = "post", route = "add/{key}")]
#[binding(name = "output1", table_name = "redirect")]
pub fn add(req: HttpRequest) -> (HttpResponse, Option<Table>) {
    let base_url = var("BaseUrl").unwrap();

    if let Ok(entity) = RedirectEntity::from_request(&req) {
        let key = req.route_params().get("key").unwrap();
        let table = add_redirect_entity(key, entity);
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
