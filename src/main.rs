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

    println!("Fetching pagination data...");

    // 1. Get the master list of page URLs
    let page_urls = client.get_pagination_urls(&args.creator, &args.sorting).await?;
    println!("Found {} pages of content.", page_urls.len());

    // 2. Loop through every page
    for (page_num, page_url) in page_urls.iter().enumerate() {
        println!("\n========================================");
        println!("Processing Page {}/{}", page_num + 1, page_urls.len());
        println!("========================================");

        // Fetch the 10 posts on this page
        let response = client.get_posts_by_url(page_url).await?;

        // 3. Loop through the posts (Your existing logic!)
        for post in response.body {
            println!("Processing Post: {} (ID: {})", post.title, post.id);

            let mut save_dir = std::path::PathBuf::from(&args.out_dir);

            if args.dir_by_post {
                let safe_title = post.title.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
                let date_str = post.published_datetime.split('T').next().unwrap_or("UnknownDate");
                let folder_name = format!("[{}] {}", date_str, safe_title);
                save_dir.push(folder_name);
            }

            tokio::fs::create_dir_all(&save_dir).await?;

            let info = client.get_post_info(&post.id).await?;

            if let Some(body) = info.body.body {
                if let Some(blocks) = body.blocks {
                    let mut media_index = 1;

                    for block in blocks.iter() {
                        // --- HANDLE IMAGES ---
                        if block.block_type == "image" && !args.skip_images {
                            if let Some(img_id) = &block.image_id {
                                if let Some(image_map) = &body.image_map {
                                    if let Some(image) = image_map.get(img_id) {
                                        let filename = format!("{:03}_{}.{}", media_index, image.id, image.extension);
                                        let filepath = save_dir.join(&filename);
                                        client.download_file(&image.original_url, &filepath, args.all).await?;
                                        media_index += 1;
                                    }
                                }
                            }
                        }
                        // --- HANDLE FILES ---
                        else if block.block_type == "file" && !args.skip_files {
                            if let Some(file_id) = &block.file_id {
                                if let Some(file_map) = &body.file_map {
                                    if let Some(file) = file_map.get(file_id) {
                                        let filename = format!("{:03}_{}.{}", media_index, file.name, file.extension);
                                        let filepath = save_dir.join(&filename);
                                        client.download_file(&file.url, &filepath, args.all).await?;
                                        media_index += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                println!("  [LOCKED] Skipping media extraction. You likely do not have the required pledge tier.");
            }
            println!("---");
        }
    }

    println!("\nFinished downloading all pages!");
    Ok(())
}