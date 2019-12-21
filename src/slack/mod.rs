use crate::table::entities::RedirectEntity;
use crate::table::{add_redirect, delete_redirect, get_redirect, update_redirect};
use log::error;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value as JsonValue};
use std::env::var;

pub async fn handle_slack_command(slack: &SlackCommand) {
    let token = var("SlackCommandToken").unwrap();
    if token == slack.token {
        if let Some(command) = VarShowCommand::parse(&slack) {
            handle_varshow_comand(command).await;
        } else {
            post_json(
                &slack.response_url,
                json!({ "text": 
"Hey 🚀! These commands are supported:
/varshow add {url} {key}
/varshow add {url}
/varshow update {url} {key}
/varshow delete {key}"}),
            )
            .await
        }
    } else {
        error!("Invalid Slack token.");
    }
}

async fn handle_varshow_comand(command: VarShowCommand) {
    let base_url = var("BaseUrl").unwrap();

    match command.task {
        VarShowTask::Help => {
            post_json(
                &command.response_url,
                json!({ "text": 
"Hey 🚀! These commands are supported:
/varshow add {url} {key}
/varshow add {url}
/varshow update {url} {key}
/varshow delete {key}"}),
            )
            .await
        }
        VarShowTask::Add(redirect_url, key_option) => {
            let key: String;
            match key_option {
                Some(k) => key = k,
                None => key = thread_rng().sample_iter(&Alphanumeric).take(10).collect(),
            }

            match get_redirect("with_key", key.as_str()).await {
                Some(existing) => {
                    let message = format!(
                        "Could not add {}/{}, it already points to {}",
                        base_url, key, existing.redirect_url
                    );
                    post_json(&command.response_url, json!({ "text": message })).await;
                }
                None => {
                    add_redirect(
                        "with_key".to_string(),
                        key.clone(),
                        RedirectEntity {
                            redirect_url: redirect_url.clone(),
                            creator: Some(command.creator),
                        },
                    )
                    .await;
                    let message =
                        format!("Added {}/{} pointing to {}", base_url, key, redirect_url);
                    post_json(&command.response_url, json!({ "text": message })).await;
                }
            }
        }
        VarShowTask::Update(redirect_url, key) => {
            update_redirect(
                "with_key".to_string(),
                key.clone(),
                RedirectEntity {
                    redirect_url: redirect_url.clone(),
                    creator: Some(command.creator),
                },
            )
            .await;
            let message = format!("Updated {}/{} pointing to {}", base_url, key, redirect_url);
            post_json(&command.response_url, json!({ "text": message })).await;
        }
        VarShowTask::Delete(key) => {
            delete_redirect("with_key".to_string(), key.clone()).await;
            let message = format!("Deleted redirect with key: {}", key);
            post_json(&command.response_url, json!({ "text": message })).await;
        }
        _ => post_json(&command.response_url, json!({ "text": "Unknown command." })).await,
    }
}

async fn post_json(url: &str, json: JsonValue) {
    let client = Client::new();
    client
        .post(url)
        .json(&json)
        .send()
        .await
        .expect("JSON should be posted.");
}

#[derive(Deserialize)]
pub struct SlackCommand {
    pub token: String,
    pub team_id: String,
    pub team_domain: String,
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
}

pub enum VarShowTask {
    Help,
    Add(String, Option<String>),
    Update(String, String),
    Delete(String),
    List,
}

impl VarShowTask {
    pub fn parse(command_text: &str) -> Option<VarShowTask> {
        let re = Regex::new(r"^\s*(?P<task>help|add|update|delete|list)\s*").unwrap();
        if let Some(task) = re
            .captures(command_text)
            .and_then(|cap| cap.name("task").map(|task| task.as_str()))
        {
            return match task {
                "help" => Some(VarShowTask::Help),
                "list" => Some(VarShowTask::List),
                "add" => {
                    let add_re = Regex::new(r"^\s*(add)\s+(?P<url>\S+)\s*(?P<key>\S*)").unwrap();
                    let args = add_re.captures(command_text).map(|cap| {
                        (
                            cap.name("url").map(|url| String::from(url.as_str())),
                            cap.name("key").map(|key| String::from(key.as_str())),
                        )
                    });
                    match args {
                        Some((Some(url), key_option)) => Some(VarShowTask::Add(url, key_option)),
                        _ => None,
                    }
                }
                "update" => {
                    let add_re = Regex::new(r"^\s*(update)\s+(?P<url>\S+)\s+(?P<key>\S+)").unwrap();
                    let args = add_re.captures(command_text).map(|cap| {
                        (
                            cap.name("url").map(|url| String::from(url.as_str())),
                            cap.name("key").map(|key| String::from(key.as_str())),
                        )
                    });
                    match args {
                        Some((Some(url), Some(key))) => Some(VarShowTask::Update(url, key)),
                        _ => None,
                    }
                }
                "delete" => {
                    let add_re = Regex::new(r"^\s*(delete)\s+(?P<key>\S+)").unwrap();
                    let args = add_re
                        .captures(command_text)
                        .and_then(|cap| cap.name("key").map(|key| String::from(key.as_str())));
                    match args {
                        Some(key) => Some(VarShowTask::Delete(key)),
                        _ => None,
                    }
                }
                _ => None,
            };
        }
        None
    }
}

pub struct VarShowCommand {
    pub task: VarShowTask,
    pub creator: String,
    pub response_url: String,
}

impl VarShowCommand {
    pub fn parse(slack: &SlackCommand) -> Option<VarShowCommand> {
        if let Some(task) = VarShowTask::parse(&slack.text) {
            return Some(VarShowCommand {
                task,
                creator: slack.user_name.clone(),
                response_url: slack.response_url.clone(),
            });
        }

        None
    }
}
