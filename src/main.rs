use anyhow::Result;  // Importa tipo Result para tratamento de erros
use clap::Parser;    // Para parser de argumentos da linha de comando

// Importa módulos locais
mod wallet;
mod bitcoin_rpc;
mod cli;

use crate::cli::{Cli, Commands};  // Importa estruturas da CLI

fn main() -> Result<()> {
    let cli = Cli::parse();  // Parse os argumentos da linha de comando

    // Match para executar o comando apropriado
    match cli.command {
        // Se o comando for GenerateAddress
        Commands::GenerateAddress { count } => {
            // Gera 'count' endereços
            for _ in 0..count {
                let address = wallet::generate_address()?;
                println!("Novo endereço gerado: {}", address);
            }
        }
        
        // Se o comando for CheckBalance
        Commands::CheckBalance { address } => {
            // Consulta o saldo do endereço
            let balance = bitcoin_rpc::get_balance(&address)?;
            println!("Saldo do endereço {}: {} BTC", address, balance);
        }
    }
    Ok(())
}