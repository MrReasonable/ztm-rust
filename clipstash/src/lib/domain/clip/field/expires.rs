use std::str::FromStr;

use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

use crate::domain::{clip::ClipError, time::Time};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T: Into<Option<Time>>>(expires: T) -> Self {
        Self(expires.into())
    }

    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl FromStr for Expires {
    type Err = ClipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            Ok(Self(None))
        } else {
            match Time::from_str(s) {
                Ok(time) => Ok(Self::new(time)),
                Err(e) => Err(e.into()),
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Expires {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        if field.value.trim().is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self::from_str(field.value)
                .map_err(|e| form::Error::validation(format!("{}", e)))?)
        }
    }
}
