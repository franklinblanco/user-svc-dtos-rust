use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

pub const AUTH_TOKEN_EXPIRATION_TIME_IN_DAYS:i32 = 1;
pub const REFRESH_TOKEN_EXPIRATION_TIME_IN_DAYS: i32 = 20;

#[derive(Serialize, Deserialize)]
pub struct Token {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i32,
    pub user_id: i32,
    #[serde(skip_serializing, skip_deserializing)]
    pub time_created: Option<NaiveDateTime>,
    #[serde(skip_serializing, skip_deserializing)]
    pub last_updated: Option<NaiveDateTime>,
    pub auth_token: String,
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