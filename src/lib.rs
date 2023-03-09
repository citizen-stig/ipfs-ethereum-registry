use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod cli;
mod ipfs;
mod blockchain;


pub async fn upload_and_register(file_path: &Path, eth_key: &str, contract_address: &str) -> anyhow::Result<(String, String)> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let content_id = ipfs::upload_data(buf_reader).await.map_err(|err| {
        log::warn!("Failed to upload data to ipfs: {:?}", err);
        err
    })?;
    log::info!("File {} has been uploaded to ipfs with CID: {}", file_path.display(), content_id);

    let tx_hash = blockchain::register_content_id(eth_key, &content_id, contract_address).await
        .map_err(|err| {
            log::warn!("Failed to submit data to blockchain: {:?}", err);
            err
        })?;
    log::info!("ContentID {} has been registered in smart contract in tx {}", content_id, tx_hash);
    Ok((content_id, tx_hash))
}