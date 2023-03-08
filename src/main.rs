use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use clap::Parser;


mod cli;
mod ipfs;
mod blockchain;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = cli::Args::parse();

    let path = Path::new(&args.path);
    if !path.exists() || !path.is_file() {
        log::error!("Path {} is not exists or a file", path.display());
        exit(1)
    }
    log::info!("Starting uploading file {}", path.display());

    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let content_id = ipfs::upload_data(buf_reader).await?;

    blockchain::register_content_id(args.eth_key, content_id).await
}