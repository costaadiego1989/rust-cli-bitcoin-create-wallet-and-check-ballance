use clap::{Parser, Subcommand};

// Define estrutura principal da CLI
#[derive(Parser)]
#[command(name = "bitcoin-wallet")]
#[command(about = "Uma CLI para gerenciar carteiras Bitcoin")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,  // Enum com comandos disponíveis
}

// Define subcomandos disponíveis
#[derive(Subcommand)]
pub enum Commands {
    // Comando para gerar endereços
    GenerateAddress {
        #[arg(short, long, default_value = "1")]
        count: u32,  // Número de endereços a gerar
    },
    // Comando para verificar saldo
    CheckBalance {
        #[arg(short, long)]
        address: String,  // Endereço a consultar
    },
}