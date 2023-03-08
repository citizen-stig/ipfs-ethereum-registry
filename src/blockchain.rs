use std::str::FromStr;
use secp256k1::SecretKey;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::types::Address;


pub async fn register_content_id(eth_key: String, content_id: String) -> anyhow::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);

    // Load the contract ABI
    let contract_abi = include_bytes!("./abi.json");
    let contract = Contract::from_json(
        web3.eth(),
        Address::from_str("0x5FbDB2315678afecb367f032d93F642f64180aa3").unwrap(),
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