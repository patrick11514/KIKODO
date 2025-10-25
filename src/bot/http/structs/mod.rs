use octocrab::models::events::payload::PullRequestEventPayload;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebhookEvent {
    PullRequest(PullRequestEventPayload),
    //TODO
}
