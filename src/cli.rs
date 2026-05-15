use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Args {
    /// The Fanbox Creator ID to download from
    #[arg(short, long)]
    pub creator: String,

    /// FANBOXSESSID cookie value for authentication
    #[arg(short, long)]
    pub session_id: String,

    /// The directory to save downloads to
    #[arg(short, long, default_value = "./downloads")]
    pub out_dir: String,

    /// Separates content saved into directories based on the title of the post
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub dir_by_post: bool,

    /// Ensure all content is downloaded, redownloading existing local content
    #[arg(long)]
    pub all: bool,

    /// Skip downloading non-image files from creators
    #[arg(long)]
    pub skip_files: bool,

    /// Skip downloading images from creators
    #[arg(long)]
    pub skip_images: bool,

    /// Sort downloads by Newest or Oldest
    #[arg(long, value_enum, default_value_t = Sorting::Newest)]
    pub sorting: Sorting,

    /// Automatically rotate NordVPN servers when rate-limited
    #[arg(long)]
    pub auto_vpn: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Sorting {
    Newest,
    Oldest,
}