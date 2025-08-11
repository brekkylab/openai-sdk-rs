use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileObject {
    pub id: String,
    pub object: String,
    pub bytes: u64,
    pub created_at: u64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileListResponse {
    pub object: String,
    pub data: Vec<FileObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDeleteResponse {
    pub id: String,
    pub object: Option<String>,
    pub deleted: bool,
}
