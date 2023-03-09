use std::env;
use std::io::Read;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient, TryFromUri};


pub async fn upload_data<R>(data: R) -> anyhow::Result<String>
    where
        R: 'static + Read + Send + Sync + Unpin,
{
    let ipfs_url = env::var("IPFS_URL").unwrap_or("http://127.0.0.1:5001".to_string());
    let client = IpfsClient::from_str(&ipfs_url)?;
    let result = client.add(data).await?;
    log::info!("Add response: name={} size={} hash={}", result.name, result.size, result.hash);
    Ok(result.hash)
}