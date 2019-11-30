use azure_functions::bindings::{HttpRequest, HttpResponse, Table};
use azure_functions::func;
use azure_functions::http::Status;

use super::entities::RedirectEntity;

#[func]
#[binding(name = "_req", route = "redirect/{key}", auth_level = "anonymous")]
#[binding(
    name = "table",
    table_name = "redirect",
    partition_key = "with_key",
    row_key = "{key}"
)]
pub fn redirect(_req: HttpRequest, table: Table) -> HttpResponse {
    let redirect_url = get_redirect_url(table);
    match redirect_url {
        None => HttpResponse::build()
            .status(Status::NotFound)
            .body("Could not find redirect key")
            .finish(),
        Some(url) => HttpResponse::build()
            .status(Status::Found)
            .header("Location", url)
            .body("The requested resource has moved.")
            .finish(),
    }
}

fn get_redirect_url(table: Table) -> Option<String> {
    let row = table.as_value().get(0)?;
    let entity: RedirectEntity = serde_json::from_value(row.clone()).unwrap();
    Some(entity.redirect_url)
}
