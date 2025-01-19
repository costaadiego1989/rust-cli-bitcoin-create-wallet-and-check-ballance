use anyhow::Result;  // Importa tipo Result para tratamento de erros
use bitcoin::Address;
use secp256k1::{ rand::rngs::OsRng, Secp256k1 }

pub fn generate_address() -> Result<String> {

    // Criar uma nova instância do Secp256k1    
    let secp = Secp256k1::new();

    // Inicializ gerador de números aleatórios
    let mut rng = OsRng::new();

    // Gerar par de chaves
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    // Criar endereço Bitcoin P2PKH a partir da chave pública
    let address = Address::p2pkh(&public_key, bitcoin::Network::Bitcoin);

    // Retorna o endereço como string
    Ok(address.to_string());

}