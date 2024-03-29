use serde::{Serialize, Deserialize};

use crate::domain::user::credential_type::CredentialType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserForCreationDto{
    #[serde(default = "get_default_app")]
    pub app: String,
    pub credential: String,
    #[serde(rename = "credentialType")]
    pub credential_type: CredentialType,
    pub password: String,
    pub name: String
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserForLoginDto {
    #[serde(default = "get_default_app")]
    pub app: String,
    pub credential: String,
    #[serde(rename = "credentialType")]
    pub credential_type: CredentialType,
    pub password: String
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserForAuthenticationDto{
    #[serde(default = "get_default_app")]
    pub app: String,
    pub id: String,
    pub token: String
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserAuthHeader{
    pub id: String,
    pub token: String
}
impl From<UserAuthHeader> for UserForAuthenticationDto {
    fn from(value: UserAuthHeader) -> Self {
        Self { app: get_default_app(), id: value.id, token: value.token }
    }
}



fn get_default_app() -> String {
    "deez".to_string()
}