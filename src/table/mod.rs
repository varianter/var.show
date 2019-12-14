pub mod entities;
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_table::table::TableService;
use derive_more::From;
use log::{error, info};
use regex::Regex;
use std::env::var;

pub async fn get_redirect(partition_key: &str, row_key: &str) -> Option<entities::RedirectEntity> {
    if let Some(ts) = get_table_service() {
        match ts
            .get_entity::<entities::RedirectEntity>("redirect", partition_key, row_key)
            .await
        {
            Ok(entity) => entity,
            Err(err) => {
                error!("Failed when trying to retrieve entity: {}", err);
                None
            }
        }
    } else {
        error!("Failed to retrieve table service");
        None
    }
}

pub async fn add_redirect(entity: entities::RedirectEntity) {
    if let Some(ts) = get_table_service() {
        match ts.insert_entity("redirect", &entity).await {
            Ok(_) => {
                info!("Updated entity.");
            }
            Err(err) => {
                error!("Failed when trying to update entity: {}", err);
            }
        }
    } else {
        error!("Failed to retrieve table service");
    }
}

pub async fn update_redirect(entity: entities::RedirectEntity) {
    if let Some(ts) = get_table_service() {
        match ts
            .update_entity("redirect", &entity.PartitionKey, &entity.RowKey, &entity)
            .await
        {
            Ok(_) => {
                info!("Inserted entity.");
            }
            Err(err) => {
                error!("Failed when trying to insert entity: {}", err);
            }
        }
    } else {
        error!("Failed to retrieve table service");
    }
}

fn get_table_service() -> Option<TableService> {
    match get_storage_client() {
        Ok(client) => Some(TableService::new(client)),
        Err(err) => {
            log_storage_client_error(err);
            None
        }
    }
}

fn get_storage_client() -> Result<Client, StorageClientError> {
    let connection_string = var("AzureWebJobsStorage")?;
    let re = Regex::new(r"AccountName=(?P<name>\S*)?;AccountKey=(?P<key>\S+);")?;
    let connection_string_matches = re
        .captures_iter(connection_string.as_str())
        .nth(0)
        .ok_or_else(|| "No match in connection string regex".to_string())?;
    let account = connection_string_matches
        .name("name")
        .ok_or_else(|| "No account found in regex match".to_string())?
        .as_str();
    let key = connection_string_matches
        .name("key")
        .ok_or_else(|| "No key found in regex match".to_string())?
        .as_str();
    let client = Client::new(account, key)?;
    Ok(client)
}

#[derive(From)]
enum StorageClientError {
    Regex(regex::Error),
    String(String),
    Var(std::env::VarError),
    Azure(azure_sdk_core::errors::AzureError),
}

fn log_storage_client_error(error: StorageClientError) {
    match error {
        StorageClientError::Azure(azure_error) => {
            error!("Azure error when retrieving client: {}", azure_error)
        }
        StorageClientError::Regex(regex_error) => {
            error!("Regex error when retrieving client: {}", regex_error)
        }
        StorageClientError::Var(var_error) => {
            error!("Env var error when retrieving client: {}", var_error)
        }
        StorageClientError::String(string_error) => {
            error!("Error when retrieving storage client: {}", string_error)
        }
    };
}
