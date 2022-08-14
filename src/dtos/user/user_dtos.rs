use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserForCreationDto{
    pub app: String,
    pub email: String,
    pub password: String,
    pub name: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserForLoginDto{
    pub app: String,
    pub email: String,
    pub password: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserForAuthenticationDto{
    pub app: String,
    pub email: String,
    pub token: String
}