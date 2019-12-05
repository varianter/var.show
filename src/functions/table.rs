use azure_functions::bindings::Table;
use serde_json::Value;

pub fn add_redirect_url(key: &String, redirect_url: String) -> Table {
    let mut table = Table::new();
    {
        let row = table.add_row("with_key", key);
        row.insert("redirect_url".to_string(), Value::String(redirect_url));
    }
    table
}
