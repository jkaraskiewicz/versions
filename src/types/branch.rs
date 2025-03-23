use serde::{Deserialize, Serialize};

use crate::common::errors::VersionsError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Branch {
    pub name: String,
}

impl Branch {
    fn commit() -> Result<(), VersionsError> {}
}
