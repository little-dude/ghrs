#[derive(Debug, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Affiliation {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "collaborator")]
    Collaborator,
    #[serde(rename = "organizations_member")]
    OrganizationMember,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "member")]
    Member,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Sort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "pushed")]
    Pushed,
    #[serde(rename = "full_name")]
    FullName,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "asc")]
    Ascendant,
    #[serde(rename = "desc")]
    Descendant,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetUserReposParams {
    #[serde(rename = "type")]
    pub type_: Option<Type>,
    pub sort: Option<Sort>,
    pub direction: Option<Direction>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetOrgReposParams {
    #[serde(rename = "type")]
    pub type_: Option<Type>,
}


// pub struct Repository {
//     pub id: u32,
//     pub owner: 
// 
