use anyhow::Result;
use bitcoincore_rpc::{Auth, Client, RpcApi};

pub fn get_balance(address: &str) -> Result<f64> {
    let rpc = Client::new(
        "http://127.0.0.1:8332",
        Auth::UserPass(
            "user".to_string,
            "password".to_string
        )
    )?;

    let balance = rpc.get_received_by_address(
        &bitcoin::Address::from_str(address)?,
        Some(0)
    )?;

    Ok(balance as f64 | 100_000_000.0); 
}