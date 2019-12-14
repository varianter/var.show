use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct RedirectEntity {
    pub RowKey: String,
    pub PartitionKey: String,
    pub redirect_url: String,
    pub creator: Option<String>,
}
