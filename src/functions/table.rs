use super::entities::RedirectEntity;
use azure_functions::bindings::Table;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::Value;

pub fn add_redirect_entity_random_key(entity: RedirectEntity) -> (String, Table) {
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    let table = add_redirect_entity(&rand_string, entity);
    (rand_string, table)
}

pub fn add_redirect_entity(key: &str, entity: RedirectEntity) -> Table {
    let mut table = Table::new();
    {
        let row = table.add_row("with_key", key);
        row.insert(
            "redirect_url".to_string(),
            Value::String(entity.redirect_url),
        );
        if let Some(creator) = entity.creator {
            row.insert("creator".to_string(), Value::String(creator));
        }
    }
    table
}
