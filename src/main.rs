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

    println!("Successfully fetched post list. Starting downloads...\n");

    for post in response.body {
        println!("Processing Post: {} (ID: {})", post.title, post.id);

        // 1. Determine the save directory for this post
        let mut save_dir = std::path::PathBuf::from(&args.out_dir);

        if args.dir_by_post {
            // Clean the title of illegal folder characters
            let safe_title = post.title.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");

            // Extract the "YYYY-MM-DD" part from "YYYY-MM-DDTHH:MM:SS+TZ:TZ"
            // This is Rust's version of python's string.split('T')[0]
            let date_str = post.published_datetime
                .split('T')
                .next()
                .unwrap_or("UnknownDate");

            // Format "[YYYY-MM-DD] POSTNAME"
            let folder_name = format!("[{}] {}", date_str, safe_title);

            save_dir.push(folder_name);
        }

        // 2. Create the directory
        tokio::fs::create_dir_all(&save_dir).await?;

        // 3. Fetch the media URLs for this specific post
        let info = client.get_post_info(&post.id).await?;

        // 4. Extract the media sequentially using 'blocks'
        if let Some(body) = info.body.body {

            // Ensure the post actually has blocks to read
            if let Some(blocks) = body.blocks {

                // Create a manual counter that only increments for media
                let mut media_index = 1;

                // We drop .enumerate() and just loop over the blocks
                for block in blocks.iter() {

                    // --- HANDLE IMAGES ---
                    if block.block_type == "image" && !args.skip_images {
                        if let Some(img_id) = &block.image_id {
                            if let Some(image_map) = &body.image_map {
                                if let Some(image) = image_map.get(img_id) {
                                    // Use our custom media_index
                                    let filename = format!("{:03}_{}.{}", media_index, image.id, image.extension);
                                    let filepath = save_dir.join(&filename);

                                    client.download_file(&image.original_url, &filepath, args.all).await?;

                                    // Increment the counter only after handling an image
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
                                    // Use our custom media_index
                                    let filename = format!("{:03}_{}.{}", media_index, file.name, file.extension);
                                    let filepath = save_dir.join(&filename);

                                    client.download_file(&file.url, &filepath, args.all).await?;

                                    // Increment the counter only after handling a file
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

    println!("Finished downloading!");
    Ok(())
}