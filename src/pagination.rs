use serde::Deserialize;

use crate::error::{ApiError, bad};

#[derive(Deserialize)]
pub(crate) struct Pagination {
    limit: Option<i64>,
    offset: Option<i64>,
}

impl Pagination {
    pub(crate) fn values(&self) -> Result<(i64, i64), ApiError> {
        let limit = self.limit.unwrap_or(20);
        let offset = self.offset.unwrap_or(0);

        if !(1..=100).contains(&limit) || offset < 0 {
            return Err(bad("limit must be 1–100; offset must be nonnegative"));
        }

        Ok((limit, offset))
    }
}
