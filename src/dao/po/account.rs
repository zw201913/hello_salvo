use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Account {
    pub id: String,
    pub account: String,
    pub password: String,
    pub enabled: i32,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}