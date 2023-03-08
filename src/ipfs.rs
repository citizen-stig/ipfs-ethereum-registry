use std::io::Read;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};

pub async fn upload_data<R>(data: R) -> anyhow::Result<String>
    where
        R: 'static + Read + Send + Sync + Unpin,
{
    let client = IpfsClient::default();
    let result = client.add(data).await?;
    log::info!("Add response: name={} size={} hash={}", result.name, result.size, result.hash);
    Ok(result.hash)
}