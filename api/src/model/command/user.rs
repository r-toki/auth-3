use crate::lib::{
    date_time::get_current_date_time, error::Error, id::get_new_id, password_hashing::hash,
};

use chrono::{DateTime, Utc};
use derive_new::new;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::json;
use validator::Validate;

lazy_static! {
    static ref RE_NAME: Regex = Regex::new(r"[A-Za-z\d#$@!%&*?]{3,15}").unwrap();
    static ref RE_PASSWORD: Regex = Regex::new(r"[A-Za-z\d#$@!%&*?]{8,30}").unwrap();
}

#[derive(new, Debug, Validate)]
pub struct User {
    pub id: String,
    #[validate(regex(
        path = "RE_NAME",
        message = "must be 3-15 characters in alphabet, numbers or symbols"
    ))]
    pub name: String,
    pub password_hash: String,
    pub refresh_token_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn create(name: String, password: String) -> Result<Self, Error> {
        if !RE_PASSWORD.is_match(&password) {
            return Err(Error::UnprocessableEntity(
                json!({"errors": {"password": ["must be 8-30 characters in alphabet, numbers or symbols"]}}),
            ));
        };

        let id = get_new_id();
        let now = get_current_date_time();

        let user = User::new(id, name, hash(&password), None, now, now);
        user.validate()?;

        Ok(user)
    }
}
