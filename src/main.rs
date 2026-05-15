mod cli;
mod models;
mod api;

use clap::Parser;
use cli::{Args, Sorting}; // Notice we import Sorting here too
use api::FanboxClient;
use tokio::process::Command;

async fn rotate_nordvpn() -> std::io::Result<()> {
    println!("\n  [VPN] Rate limit detected. Initiating NordVPN rotation...");

    println!("  [VPN] Disconnecting...");
    let _ = Command::new("nordvpn")
        .arg("-d")
        .status()
        .await?;

    println!("  [VPN] Connecting to a new server...");
    let _ = Command::new("nordvpn")
        .arg("-c")
        .arg("-g")
        .arg("United States")
        .status()
        .await?;

    println!("  [VPN] Stabilizing network connection...");
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    println!("  [VPN] Rotation complete. Resuming downloads...\n");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Starting download for creator: {}", args.creator);
    println!("Output directory: {}", args.out_dir);

    let client = FanboxClient::new(&args.session_id)?;

    println!("Fetching pagination data...");

    let page_urls = client.get_pagination_urls(&args.creator, &args.sorting).await?;
    println!("Found {} pages of content.", page_urls.len());

    for (page_num, page_url) in page_urls.iter().enumerate() {
        println!("\n========================================");
        println!("Processing Page {}/{}", page_num + 1, page_urls.len());
        println!("========================================");

        let response = client.get_posts_by_url(page_url).await?;

        for post in response.body {
            println!("Processing Post: {} (ID: {})", post.title, post.id);

            let mut save_dir = std::path::PathBuf::from(&args.out_dir);

            if args.dir_by_post {
                let safe_title = post.title
                    .replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\n', '\r', '\t'], "_") // Replaces illegal chars AND newlines with underscores
                    .trim() // Removes leading and trailing spaces
                    .trim_end_matches('.') // Removes trailing periods (another Windows path killer)
                    .to_string();
                let date_str = post.published_datetime.split('T').next().unwrap_or("UnknownDate");
                let folder_name = format!("[{}] {}", date_str, safe_title);
                save_dir.push(folder_name);
            }

            tokio::fs::create_dir_all(&save_dir).await?;

            let max_retries = 10;
            let mut current_try = 0;

            let info_result = loop {
                match client.get_post_info(&post.id).await {
                    Ok(data) => break Some(data),
                    Err(e) => {
                        current_try += 1;
                        if current_try >= max_retries {
                            eprintln!("  [ERROR] Failed to fetch Post {} after {} attempts. Error: {}", post.id, max_retries, e);
                            break None;
                        }

                        eprintln!("  [WARNING] Error fetching Post {} (Attempt {}/{}): {}", post.id, current_try, max_retries, e);

                        if args.auto_vpn && current_try >= 2{
                            if let Err(vpn_err) = rotate_nordvpn().await {
                                eprintln!("  [VPN ERROR] Failed to rotate VPN: {}", vpn_err);
                            }
                        } else {
                            println!("  Retrying in 5 second...");
                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        }
                    }
                }
            };

            let info = match info_result {
                Some(data) => data,
                None => {
                    println!("---");
                    continue;
                }
            };

            if let Some(body) = info.body.body {
                if let Some(blocks) = body.blocks {
                    let mut media_index = 1;

                    for block in blocks.iter() {
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