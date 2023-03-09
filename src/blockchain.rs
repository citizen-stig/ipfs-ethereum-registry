use std::str::FromStr;
use std::env;
use secp256k1::SecretKey;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::types::Address;


pub async fn register_content_id(eth_key: String, content_id: String, contract_address: String) -> anyhow::Result<()> {
    let url = env::var("BLOCKCHAIN_URL").unwrap_or("http://localhost:8545".to_string());
    let transport = web3::transports::Http::new(&url)?;
    let web3 = web3::Web3::new(transport);

    let contract_abi = include_bytes!("./abi.json");
    let contract = Contract::from_json(
        web3.eth(),
        Address::from_str(&contract_address).unwrap(),
        contract_abi,
    )?;

    let secret_key = SecretKey::from_str(&eth_key)?;
    let sk = SecretKeyRef::new(&secret_key);

    let tx = contract.signed_call(
        "registerFile",
        (content_id, ),
        Options::default(),
        sk,
    ).await?;

    log::info!("File was registered in transaction with hash: {:?}", tx);

    Ok(())
}