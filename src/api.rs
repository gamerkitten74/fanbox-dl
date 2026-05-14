use reqwest::header::{ACCEPT, COOKIE, ORIGIN, USER_AGENT, HeaderMap};
use crate::models::FanboxResponse;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use crate::models::PostInfoResponse;

pub struct FanboxClient {
    client: reqwest::Client,
}

impl FanboxClient {
    // This is like Python's __init__
    pub fn new(session_id: &str) -> Result<Self, reqwest::Error> {
        let mut headers = HeaderMap::new();

        let cookie_str = format!("FANBOXSESSID={}", session_id);
        headers.insert(COOKIE, cookie_str.parse().unwrap());
        headers.insert(ORIGIN, "https://www.fanbox.cc".parse().unwrap());
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36".parse().unwrap(),
        );
        headers.insert(ACCEPT, "application/json, text/plain, */*".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }

    // Add this new method to fetch the master list of page URLs
    pub async fn get_pagination_urls(
        &self,
        creator: &str,
        sorting: &crate::cli::Sorting
    ) -> Result<Vec<String>, reqwest::Error> {
        // Convert our Enum into the string Fanbox expects
        let sort_str = match sorting {
            crate::cli::Sorting::Newest => "newest",
            crate::cli::Sorting::Oldest => "oldest",
        };

        let url = format!("https://api.fanbox.cc/post.paginateCreator?creatorId={}&sort={}", creator, sort_str);

        let response = self.client.get(&url)
            .send()
            .await?
            .json::<crate::models::PaginateResponse>()
            .await?;

        Ok(response.body)
    }

    // Modify this to accept a pre-built URL instead of building it
    pub async fn get_posts_by_url(&self, page_url: &str) -> Result<crate::models::FanboxResponse, reqwest::Error> {
        let response = self.client
            .get(page_url)
            .send()
            .await?
            .json::<crate::models::FanboxResponse>()
            .await?;

        Ok(response)
    }

    // Fetch the detailed post info
    pub async fn get_post_info(&self, post_id: &str) -> Result<PostInfoResponse, reqwest::Error> {
        let url = format!("https://api.fanbox.cc/post.info?postId={}", post_id);
        let response = self.client.get(&url).send().await?.json::<PostInfoResponse>().await?;
        Ok(response)
    }

    // The universal download function
    pub async fn download_file(
        &self,
        url: &str,
        filepath: &Path,
        overwrite: bool
    ) -> Result<(), Box<dyn std::error::Error>> {

        // Skip logic: if the file exists and --all was NOT passed, return early.
        if filepath.exists() && !overwrite {
            println!("  [SKIP] Already exists: {:?}", filepath.file_name().unwrap());
            return Ok(());
        }

        println!("  [DOWNLOADING] {:?}", filepath.file_name().unwrap());

        // 1. Fetch the file
        let response = self.client.get(url).send().await?.error_for_status()?;

        // 2. Load it into memory (bytes)
        let bytes = response.bytes().await?;

        // 3. Create the file and write the bytes asynchronously
        let mut file = tokio::fs::File::create(filepath).await?;
        file.write_all(&bytes).await?;

        Ok(())
    }
}