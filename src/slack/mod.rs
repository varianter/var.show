use crate::table::entities::RedirectEntity;
use crate::table::{add_redirect, get_redirect, update_redirect};
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;

pub async fn handle_slack_command(slack: &SlackCommand) {
    if let Some(command) = VarShowCommand::parse(&slack) {
        match command.task {
            VarShowTask::Help => println!("Help!"),
            VarShowTask::Add(redirect_url, key_option) => {
                let key = key_option.unwrap();
                add_redirect(RedirectEntity {
                    RowKey: key.clone(),
                    PartitionKey: "with_key".to_string(),
                    redirect_url,
                    creator: Some(command.creator),
                })
                .await;
                let client = reqwest::Client::new();
                let full_url = format!("http://var.show/{}", key);
                client
                    .post(slack.response_url.as_str())
                    .body(full_url)
                    .send()
                    .await
                    .expect("wuut");
            }
            _ => println!("Handle rest later"),
        }
    }
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
}

impl VarShowCommand {
    pub fn parse(slack: &SlackCommand) -> Option<VarShowCommand> {
        if let Some(task) = VarShowTask::parse(&slack.text) {
            return Some(VarShowCommand {
                task,
                creator: slack.user_name.clone(),
            });
        }

        None
    }
}
