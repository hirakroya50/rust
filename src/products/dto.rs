use serde::Deserialize;

use crate::error::{ApiError, bad};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProductInput {
    pub(super) name: String,
    pub(super) description: Option<String>,
    pub(super) price: String,

    #[serde(default)]
    pub(super) stock: i32,

    pub(super) category: Option<String>,
}

impl ProductInput {
    pub(super) fn clean(mut self) -> Result<Self, ApiError> {
        self.name = self.name.trim().to_string();

        if self.name.is_empty() {
            return Err(bad("Name is required"));
        }

        if self.stock < 0 {
            return Err(bad("Stock must be nonnegative"));
        }

        let parts: Vec<&str> = self.price.split('.').collect();

        let digits = |value: &str| !value.is_empty() && value.bytes().all(|c| c.is_ascii_digit());

        let invalid = parts.len() > 2
            || !digits(parts[0])
            || parts[0].len() > 10
            || (parts.len() == 2 && (!digits(parts[1]) || parts[1].len() > 2));

        if invalid {
            return Err(bad("Price must be a decimal string, e.g. 199.99, \
                 with at most 10 integer and 2 decimal digits"));
        }

        Ok(self)
    }
}
