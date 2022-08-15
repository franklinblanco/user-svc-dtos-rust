use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};

use crate::dtos::user::user_dtos::UserForCreationDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<NaiveDateTime>,
    pub app: String,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub salt: String
}
impl User {
    pub fn _new() -> User {
        User { id: 0,
            time_created: None, //  This will be automatically generated from the database
            last_updated: None, //  This will be automatically generated from the database
            app: "".to_string(),
            email: "".to_string(),
            name:"".to_string(),
            password:"".to_string(),
            salt: "".to_string() }
    }
    pub fn new_for_creation(incoming_user: &UserForCreationDto) -> User{        
        User { id: 0,
            time_created: None, //  This will be automatically generated from the database
            last_updated: None, //  This will be automatically generated from the database
            app: incoming_user.app.to_string(),
            email: incoming_user.email.to_string(),
            name: incoming_user.name.to_string(),
            password: incoming_user.password.to_string(),
            salt: "".to_string() }
    }
}
