use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

pub const AUTH_TOKEN_EXPIRATION_TIME_IN_DAYS:i32 = 1;
pub const REFRESH_TOKEN_EXPIRATION_TIME_IN_DAYS: i32 = 20;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(skip_serializing, skip_deserializing)]
    pub time_created: Option<NaiveDateTime>,
    #[serde(skip_serializing, skip_deserializing)]
    pub last_updated: Option<NaiveDateTime>,
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String
}

impl Token{
    pub fn new(user_id: i32, auth_token: String, refresh_token: String) -> Token{
        Token { 
            id: 0,
            user_id,
            time_created: None,
            last_updated: None,
            auth_token,
            refresh_token
        }
    }
}