use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

use std::env;  

fn main() -> Result<(), anyhow::Error> {
    
    dotenv::dotenv().ok();

    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(rpc_url);

   
    let sender_private_key = env::var("PRIVATE_KEY")?;
    let sender_keypair = Keypair::from_base58_string(&sender_private_key);

    
    let recipient_address_str = env::var("RECIPIENT_SOLANA_ADDRESS")?;
    let receiver_address = Pubkey::from_str(&recipient_address_str)?;

    
    let transfer_amount = (0.1 * 1_000_000_000.0) as u64; // 1 SOL = 1,000,000,000 lamports

    let transfer_ix = system_instruction::transfer(
        &sender_keypair.pubkey(),
        &receiver_address,
        transfer_amount,
    );

    
    let recent_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&sender_keypair.pubkey()),
        &[&sender_keypair],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("SOL sent! Signature: {}", signature);

    Ok(())
}