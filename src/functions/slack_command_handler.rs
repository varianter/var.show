use super::entities::SlackPayload;
use crate::slack::{handle_slack_command, SlackCommand};
use azure_functions::bindings::QueueMessage;
use azure_functions::{bindings::QueueTrigger, func};
use serde::de::value::Error as DeserializeError;

#[func]
#[binding(name = "trigger", queue_name = "slackcommand")]
pub async fn slack_command_handler(trigger: QueueTrigger) {
    let slack_command = retrieve_slack_command(&trigger.message);
    handle_slack_command(&slack_command).await;
}

fn retrieve_slack_command(message: &QueueMessage) -> SlackCommand {
    let slack_message = SlackPayload::from_queue_message(message).unwrap();
    SlackCommand::from_str(&slack_message.payload).unwrap()
}

impl SlackCommand {
    pub fn from_str(text: &str) -> std::result::Result<SlackCommand, DeserializeError> {
        serde_urlencoded::from_str(text)
    }
}
