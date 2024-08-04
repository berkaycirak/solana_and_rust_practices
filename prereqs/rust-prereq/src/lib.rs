use solana_sdk::{
    signature::{ Keypair, Signer, read_keypair_file },
    transaction::Transaction,
    message::Message,
};
use std::str::FromStr;
use solana_program::{ pubkey::Pubkey, system_instruction::transfer, system_program };
use solana_client::rpc_client::RpcClient;
use bs58;
use std::io::{ self, BufRead };

const RPC_URL: &str = "https://api.devnet.solana.com";

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod programs;

#[cfg(test)]
mod tests {
    use crate::programs::wba_prereq::{ WbaPrereqProgram, CompleteArgs, UpdateArgs };
    use solana_sdk;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }
    #[test]
    fn airdop() {
        let keypair = read_keypair_file("wallet.json").expect("Couldn't find wallet file");

        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }
    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("wallet.json").expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("GTfyXsd3pVNvXMbHwJARvoVidzx9LSabTxnvUnyJ1Ppx").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );

        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here:
https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
    #[test]
    fn complete() {
        let signer = read_keypair_file("wallet.json").expect("Couldn't find wallet file");

        let client = RpcClient::new(RPC_URL);
        // for PDA
        let prereq = WbaPrereqProgram::derive_program_address(
            &[b"prereq", signer.pubkey().to_bytes().as_ref()]
        );

        let args = CompleteArgs {
            github: b"berkaycirak".to_vec(),
        };

        let blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );

        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        // Print our transaction out
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }
    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
