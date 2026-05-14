mod cli;
mod models;
mod api;

use clap::Parser;
use cli::Args;
use api::FanboxClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Starting download for creator: {}", args.creator);
    println!("Output directory: {}", args.out_dir);

    // Initialize our API client
    let client = FanboxClient::new(&args.session_id)?;

    // Fetch the posts
    let response = client.get_posts(&args.creator).await?;

    println!("Successfully parsed JSON!");

    for post in response.body {
        println!("Found Post ID: {}, Title: '{}', Fee: ¥{}", post.id, post.title, post.fee_required);
    }

    Ok(())
}