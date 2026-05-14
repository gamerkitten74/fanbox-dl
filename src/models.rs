use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct FanboxResponse {
    pub body: Vec<Post>,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct PostInfoResponse {
    // The root JSON only has a "body" key
    pub body: PostInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostInfo {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub post_type: String,
    
    pub body: Option<PostSpecificBody>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostSpecificBody {
    pub blocks: Option<Vec<Block>>,

    pub image_map: Option<HashMap<String, ImageItem>>,
    pub file_map: Option<HashMap<String, FileItem>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "type")]
    pub block_type: String,
    pub image_id: Option<String>,
    pub file_id: Option<String>,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageItem {
    pub id: String,
    pub extension: String,
    pub width: u32,
    pub height: u32,
    pub original_url: String,
    pub thumbnail_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileItem {
    pub id: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub url: String,
}