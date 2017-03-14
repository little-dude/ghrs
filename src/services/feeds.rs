#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Feeds {
    pub timeline_url: String,
    pub user_url: String,
    pub current_user_public_url: String,
    pub current_user_url: String,
    pub current_user_actor_url: String,
    pub current_user_organization_url: String,
    pub current_user_organization_urls: Vec<String>,
    #[serde(rename = "_links")]
    pub links: FeedLinks,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FeedLink {
    pub href: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FeedLinks {
    pub timeline: FeedLink,
    pub user: FeedLink,
    pub current_user_public: FeedLink,
    pub current_user: FeedLink,
    pub current_user_actor: FeedLink,
    pub current_user_organization: FeedLink,
    pub current_user_organizations: Vec<FeedLink>,
}
