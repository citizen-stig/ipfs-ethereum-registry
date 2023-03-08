use std::io::Read;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};

pub async fn upload_data<R>(data: R)
    where
        R: 'static + Read + Send + Sync + Unpin,
{
    let client = IpfsClient::default();
    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}