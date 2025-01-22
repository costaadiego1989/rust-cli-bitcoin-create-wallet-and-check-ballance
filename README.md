# Bitcoin Wallet CLI

Este é um projeto CLI (Command Line Interface) para gerenciar carteiras Bitcoin, incluindo a criação de carteiras e consulta de saldo de endereços Bitcoin.

## Pré-requisitos

Antes de começar, certifique-se de ter os seguintes itens instalados:

- [Rust](https://www.rust-lang.org/tools/install) (para compilar o projeto).
- Arquivo `.env` configurado com as credenciais do Bitcoin Core.

## Instalação

1. Clone este repositório: git clone https://github.com/seu-usuario/bitcoin-wallet-cli.git
   cd bitcoin-wallet-cli

2. Configure o arquivo .env com as credenciais do Bitcoin Core:

   RPC_HOST=127.0.0.1
   RPC_PORT=18443
   RPC_USER=user
   RPC_PASSWORD=pass

3. Compile o projeto: cargo build

## Comandos Disponíveis

### 1. Criar uma Carteira ou mais

Para criar uma nova carteira, execute: cargo run generate-address

Para criar uma quantidade específica de carteiras, execute: cargo run generate-address --count 5

### 2. Consultar Saldo de um Endereço

Para consultar o saldo de um endereço Bitcoin:

cargo run check-balance --address <endereco-bitcoin>
