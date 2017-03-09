use entity::Entity;
use client::{Client, AsUrlParams, EmptyUrlParams};
use entities::chrono::{DateTime, UTC};
use errors::*;
use reqwest::Response;

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

impl<'a> AsUrlParams for &'a Params {
    type I = Vec<(&'static str, String)>;
    type K = &'static str;
    type V = String;

    fn as_url_params(self) -> Self::I {
        let mut params = Vec::new();

        if let Some(all) = self.all {
            if all {
                params.push(("all", "true".to_owned()));
            } else {
                params.push(("all", "false".to_owned()));
            }
        }

        if let Some(participating) = self.participating {
            if participating {
                params.push(("participating", "true".to_owned()));
            } else {
                params.push(("participating", "false".to_owned()));
            }
        }

        if let Some(since) = self.since {
            params.push(("since", since.to_rfc3339()))
        }

        if let Some(before) = self.before {
            params.push(("before", before.to_rfc3339()))
        }

        params
    }
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
pub struct Thread<'a> {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    client: Option<&'a Client>,

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

impl<'a> Entity<'a> for Thread<'a> {
    fn set_client(&mut self, client: &'a Client) {
        self.client = Some(client);
    }

    fn get_client(&self) -> Option<&'a Client> {
        self.client
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkRead {
    last_read_at: DateTime<UTC>,
}

impl<'a> Thread<'a> {
    pub fn new(client: &'a Client) -> Self {
        let mut user: Self = Default::default();
        user.set_client(client);
        user
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn mark_read(&self) -> Result<Response> {
        self.patch(
            vec!["notifications", "thread", &self.id().to_string()],
            &MarkRead { last_read_at: UTC::now() })
    }

    pub fn get(&mut self) -> Result<Response> {
        if let Some(client) = self.get_client() {
            let path = vec!["notifications".to_owned(), "thread".to_owned(), self.id().to_string()];
            let params = EmptyUrlParams{};
            let resp = client.get_entity(&path, Some(&params), self)?;
            Ok(resp)
        } else {
            Err(ErrorKind::UninitializedEntity.into())
        }
    }
}
