use services::chrono::{DateTime, UTC};

#[derive(Default, Debug)]
pub struct Params {
    /// If true, show notifications marked as read.
    pub all: Option<bool>,
    /// If true, only shows notifications in which the user is directly participating or mentioned.
    pub participating: Option<bool>,
    /// Only show notifications updated after the given time.
    pub since: Option<DateTime<UTC>>,
    /// Only show notifications updated before the given time.
    pub before: Option<DateTime<UTC>>,
}

/// Reason for which the user got a notification.
#[derive(Debug, Serialize, Deserialize)]
pub enum Reason {
    /// The user is assigned to the issue.
    #[serde(rename = "assign")]
    Assign,
    /// The user created the thread.
    #[serde(rename = "author")]
    Author,
    /// The user commented on the thread.
    #[serde(rename = "comment")]
    Comment,
    /// The user accepted an invitation to contribute to the repository
    #[serde(rename = "invitation")]
    Invitation,
    /// The user subscribed to the thread (via an issue or a pull request).
    #[serde(rename = "manual")]
    Manual,
    /// The user was specificallly *@mentioned* in the content.
    #[serde(rename = "mention")]
    Mention,
    /// The user changed the thread state (for example, closing an issue or merging a pull.
    /// request).
    #[serde(rename = "statechange")]
    StateChange,
    /// The user is watching the repository.
    #[serde(rename = "subscribed")]
    Subscribed,
    /// The user is on a team that was mentioned.
    #[serde(rename = "teammention")]
    TeamMention,
    None,
}

impl Default for Reason {
    fn default() -> Self {
        Reason::None
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Thread {
    pub id: u32,
    pub reason: Reason,
    pub unread: bool,
    pub updated_at: Option<DateTime<UTC>>,
    pub last_read_at: Option<DateTime<UTC>>,
    pub url: String,
    pub repository: Repository,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Owner {
    pub login: String,
    pub id: u32,
    pub html_url: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Repository {
    pub owner: Owner,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub private: bool,
    pub fork: bool,
    pub html_url: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkRead {
    last_read_at: DateTime<UTC>,
}
