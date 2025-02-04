use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Error {
    pub error: bool,
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Create {
    pub username: String,
    pub vanity: String,
    pub email: String,
    pub password: String,
    pub birthdate: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub token: String,
    pub vanity: String,
}