use services::chrono::{DateTime, UTC};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub login: String,
    pub id: u32,
    pub avatar_url: String,
    pub html_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub following_url: String,
    pub followers_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub received_events_url: String,
    pub events_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // user profile
    pub login: String,
    pub id: u32,
    pub avatar_url: String,
    pub html_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub following_url: String,
    pub followers_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub received_events_url: String,
    pub events_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,

    // public infos
    pub name: String,
    pub company: String,
    pub blog: String,
    pub location: String,
    pub email: String,
    pub hireable: Option<bool>,
    pub bio: String,
    pub public_repos: u32,
    pub public_gists: u32,
    pub followers: u32,
    pub following: u32,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,

    // private infos, available only for the authenticated user
    pub total_private_repos: Option<u32>,
    pub owned_private_repos: Option<u32>,
    pub private_gists: Option<u32>,
    pub disk_usage: Option<u32>,
    pub collaborators: Option<u32>,
    pub plan: Option<Plan>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    name: String,
    space: u32,
    collaborators: u32,
    private_repos: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserPatch {
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
}
