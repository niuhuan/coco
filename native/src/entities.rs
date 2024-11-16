use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagNetworkResponse {
    pub version: i64,
    pub data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagData {
    pub version: i64,
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub image_total: i64,
    pub tag_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPage {
    pub posts: Vec<Post>,
    pub page_total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub tags: String,
    pub created_at: i64,
    pub creator_id: Option<i64>,
    pub author: String,
    pub change: i64,
    pub source: String,
    pub score: i64,
    pub md5: String,
    pub file_size: i64,
    pub file_url: String,
    pub is_shown_in_index: bool,
    pub preview_url: String,
    pub preview_width: i64,
    pub preview_height: i64,
    pub actual_preview_width: i64,
    pub actual_preview_height: i64,
    pub sample_url: String,
    pub sample_width: i64,
    pub sample_height: i64,
    pub sample_file_size: i64,
    pub jpeg_url: String,
    pub jpeg_width: i64,
    pub jpeg_height: i64,
    pub jpeg_file_size: i64,
    pub rating: String,
    pub has_children: bool,
    pub parent_id: Option<i64>,
    pub status: String,
    pub width: i64,
    pub height: i64,
    pub is_held: bool,
    pub frames_pending_string: String,
    // pub frames_pending: Vec<Value>,
    pub frames_string: String,
    // pub frames: Vec<Value>,
}
