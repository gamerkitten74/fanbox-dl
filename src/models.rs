use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FanboxResponse {
    pub body: Vec<Post>,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: String,
    pub title: String,
    #[serde(rename = "feeRequired")]
    pub fee_required: u32,
}