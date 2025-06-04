use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct HealthRecord {
    pub cid: String,
    pub patient_did: String,
    pub encrypted_metadata: JsonValue,
    pub uploaded_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
}

impl HealthRecord {}
