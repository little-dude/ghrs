use entity::Entity;
use client::Client;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User<'a> {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    client: Option<&'a Client>,

    pub login: String,
    pub id: u32,
    pub avatar_url: String,
    pub html_url: String,
    pub gravatar_id: String,
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

    // FIXME: dates should be typed
    pub created_at: String,
    pub updated_at: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,
    pub total_private_repos: u32,
    pub owned_private_repos: u32,
    pub private_gists: u32,
    pub disk_usage: u32,
    pub collaborators: u32,
    pub plan: Plan,

    // API URLs
    pub url: String,
    pub events_url: String,
    pub following_url: String,
    pub followers_url: String,
    pub gists_url: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Plan {
    name: String,
    space: u32,
    collaborators: u32,
    private_repos: u32,
}

impl<'a> Entity<'a> for User<'a> {
    fn set_client(&mut self, client: &'a Client) {
        self.client = Some(client);
    }

    fn get_client(&self) -> Option<&'a Client> {
        self.client
    }
}

impl<'a> User<'a> {
    pub fn new(client: &'a Client) -> Self {
        let mut user: Self = Default::default();
        user.set_client(client);
        user
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
