mod cli;
mod models;
mod api;

use clap::Parser;
use cli::{Args, Sorting}; // Notice we import Sorting here too
use api::FanboxClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Starting download for creator: {}", args.creator);
    println!("Output directory: {}", args.out_dir);

    // Testing out our new boolean flags
    if args.dir_by_post {
        println!("Setting: Content will be separated into directories by post.");
    }
    if args.all {
        println!("Setting: Overwriting existing files (--all).");
    }

    // Testing the Enum
    match args.sorting {
        Sorting::Newest => println!("Sorting: Processing newest posts first."),
        Sorting::Oldest => println!("Sorting: Processing oldest posts first."),
    }

    // Initialize our API client
    let client = FanboxClient::new(&args.session_id)?;

    // Fetch the posts
    let response = client.get_posts(&args.creator).await?;

    println!("Successfully parsed JSON!");

    for post in response.body {
        println!("Found Post ID: {}, Title: '{}'", post.id, post.title);
    }

    Ok(())
}