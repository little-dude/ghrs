use errors::*;
use reqwest::Response;
use entities::{User, Feeds, Thread, notifications};
use client::{Client, EmptyUrlParams};
use entity::Entity;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Root<'a> {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    client: Option<&'a Client>,
}

impl<'a> Entity<'a> for Root<'a> {
    fn set_client(&mut self, client: &'a Client) {
        self.client = Some(client);
    }

    fn get_client(&self) -> Option<&'a Client> {
        self.client
    }
}

impl<'a> Root<'a> {
    pub fn new(client: &'a Client) -> Self {
        Root { client: Some(client) }
    }

    /// Get the authenticated user.
    pub fn get_user(&self) -> Result<(Response, User)> {
        self.get_entity::<User, _, &EmptyUrlParams>(&["user"], None)
    }

    /// Get the list of Atom feeds for the authenticated user.
    pub fn get_feeds(&self) -> Result<(Response, Feeds)> {
        self.get_entity::<Feeds, _, &EmptyUrlParams>(&["feeds"], None)
    }

    pub fn get_notifications(&self,
                             params: Option<&notifications::Params>)
                             -> Result<(Response, Vec<Thread>)> {
        self.get_entities::<Thread, _, _>(&["notifications"], params)
    }
}
