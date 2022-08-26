use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialType {
    PhoneNumber,
    Email
}
impl CredentialType {
    pub fn get_max_length(&self) -> usize {
        match self {
            CredentialType::PhoneNumber => 10,
            CredentialType::Email => 255,
        }
    }
    pub fn get_regex_pattern(&self) -> &str {
        match self {
            CredentialType::PhoneNumber => "REGEX PATTERN FOR PHONE NUMBER",
            CredentialType::Email => "REGEX PATTERN FOR EMAIL",
        }
    }
}
impl Display for CredentialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CredentialType::Email => f.write_str("Email"),
            CredentialType::PhoneNumber => f.write_str("PhoneNumber")
        }
    }
}