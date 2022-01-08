use bson::DateTime;
use chrono::MIN_DATETIME;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub release: DateTime,
}

impl Default for Book {
    fn default() -> Book {
        Book {
            title: "".to_string(),
            author: "".to_string(),
            release: DateTime::from(MIN_DATETIME),
        }
    }
}