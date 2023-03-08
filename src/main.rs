use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use clap::Parser;


mod cli;
mod ipfs;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = cli::Args::parse();

    let path = Path::new(&args.path);
    if !path.exists() || !path.is_file() {
        log::error!("Path {} is not exists or a file", path.display());
        exit(1)
    }
    log::info!("Starting uploading file {}", path.display());

    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    ipfs::upload_data(buf_reader).await;
    log::info!("DONE")
}