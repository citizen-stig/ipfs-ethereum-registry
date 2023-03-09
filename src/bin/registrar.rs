use std::path::Path;
use std::process::exit;
use clap::Parser;
use ipfs_registry::{cli, upload_and_register};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = cli::Args::parse();

    let path = Path::new(&args.path);
    if !path.exists() || !path.is_file() {
        log::error!("Path {} is not exists or not a file", path.display());
        exit(1)
    }
    log::info!("Starting uploading file {}", path.display());

    let (_, _) = upload_and_register(&path, &args.eth_key, &args.contract_address).await?;
    Ok(())
}