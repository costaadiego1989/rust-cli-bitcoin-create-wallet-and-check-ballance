use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use bitcoin::address::Address;
use bitcoin::Network;
use bitcoincore_rpc::bitcoin::Denomination;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::bitcoin::Address as RpcAddress;

pub fn get_balance(address: &str) -> Result<f64> {
    dotenv().ok();

    // Lê variáveis de ambiente ou usa valores padrão
    let host = env::var("RPC_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("RPC_PORT").unwrap_or_else(|_| "18443".to_string());
    let user = env::var("RPC_USER").unwrap_or_else(|_| "user".to_string());
    let password = env::var("RPC_PASSWORD").unwrap_or_else(|_| "pass".to_string());

    let url = format!("http://{}:{}", host, port);

    // Cria o cliente RPC
    let rpc = Client::new(
        &url,
        Auth::UserPass(user, password),
    )?;

    // Converte o endereço para Address<NetworkChecked>
    let unchecked_address = Address::from_str(address)?;
    let checked_address = unchecked_address.require_network(Network::Bitcoin)?;

    // Converte bitcoin::Address para bitcoincore_rpc::bitcoin::Address
    let rpc_address = RpcAddress::from_str(&checked_address.to_string())?;

    // Obtém o saldo pelo endereço validado
    let balance = rpc.get_received_by_address(&rpc_address, Some(0))?;

    // Converte o saldo para Bitcoin (f64)
    let balance_in_btc = balance.to_float_in(Denomination::Bitcoin);

    Ok(balance_in_btc)
}
