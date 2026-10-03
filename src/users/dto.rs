use serde::Deserialize;
use validator::ValidateEmail;

use crate::error::{ApiError, bad};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UserInput {
    pub(super) name: String,
    pub(super) email: String,
    pub(super) phone: Option<String>,
}

impl UserInput {
    pub(super) fn clean(mut self) -> Result<Self, ApiError> {
        self.name = self.name.trim().to_string();
        self.email = self.email.trim().to_lowercase();

        if self.name.is_empty() {
            return Err(bad("Name is required"));
        }

        if !self.email.validate_email() {
            return Err(bad("Valid email is required"));
        }

        self.phone = self
            .phone
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        Ok(self)
    }
}
