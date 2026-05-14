use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub creator: String,

    #[arg(short, long)]
    pub session_id: String,

    #[arg(short, long, default_value = "./downloads")]
    pub out_dir: String,
}