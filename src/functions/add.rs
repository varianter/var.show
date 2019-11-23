use azure_functions::{
    bindings::{HttpRequest, Table},
    func,
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
pub fn add(req: HttpRequest) -> ((), Table) {
    if let Ok(body) = req.body().as_json::<RedirectEntity>() {
        let mut table = Table::new();
        {
            let row = table.add_row("with_key", req.route_params().get("key").unwrap());
            row.insert("redirect_url".to_string(), Value::String(body.redirect_url));
        }
        return ((), table);
    }

    panic!("Request body was invalid")
}
