use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserForCreationDto{
    #[serde(default = "get_default_app")]
    pub app: String,
    pub email: String,
    pub password: String,
    pub name: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserForLoginDto{
    #[serde(default = "get_default_app")]
    pub app: String,
    pub email: String,
    pub password: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserForAuthenticationDto{
    #[serde(default = "get_default_app")]
    pub app: String,
    pub email: String,
    pub token: String
}

fn get_default_app() -> String {
    "deez".to_string()
}