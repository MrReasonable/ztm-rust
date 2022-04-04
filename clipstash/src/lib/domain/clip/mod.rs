use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod field;

#[derive(Error, Debug)]
pub enum ClipError {
    #[error("invalid password: {0}")]
    InvalidPassword(String),
    #[error("invalid title: {0}")]
    InvalidTitle(String),
    #[error("empty content")]
    EmptyContent,
    #[error("invalid date: {0}")]
    InvalidDate(String),
    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    clip_id: field::ClipId,
    shortcode: field::ShortCode,
    content: field::Content,
    title: field::Title,
    posted: field::Posted,
    expires: field::Expires,
    password: field::Password,
    hits: field::Hits,
}
