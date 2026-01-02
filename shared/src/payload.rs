use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ImagePayload {
    pub uri: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FilePathPayload {
    pub paths: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FilePayload {
    pub path: String,
}
