use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ImagePayload {
    pub uri: String,
}
