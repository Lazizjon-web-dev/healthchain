use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub did: String,
    pub encrypted_email: Vec<u8>,
    pub argon2_hash: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}

impl User {}
