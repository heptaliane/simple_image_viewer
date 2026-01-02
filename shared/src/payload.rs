use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct FilePayload {
    pub path: String,
}
