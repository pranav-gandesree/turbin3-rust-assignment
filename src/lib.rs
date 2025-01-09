
mod programs;

#[cfg(test)]
mod tests {

    use solana_sdk::{
        message::Message,
        signature::{read_keypair_file, Keypair, Signer}, 
        system_program,
        transaction::Transaction
    };
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use std::str::FromStr;

    use crate::programs::Turbin3_prereq::{ CompleteArgs, Turbin3PrereqProgram };
    const RPC_URL: &str = "https://api.devnet.solana.com";


    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You have generated a solana wallet:{}",
            kp.pubkey().to_string()
        );

        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }






    #[test] 
    fn airdop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(success) => {
                println!("check your transaction here");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    success.to_string()
                );
            }
            Err(err) => println!("transaction failed : {}", err.to_string()),
        }
    }




    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("3SGxRfayf5VHKiqYVSPACrxQu17LPkFXGv8yXznbfjhE").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent
        blockhash");

        let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );

        let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("failed to get fee calculator");


        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("transaction failed!");
        println!(
            "Success! Check out your TX here : https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }



    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(
            &[b"prereq", signer.pubkey().to_bytes().as_ref()]
        );

        let args = CompleteArgs {
            github: b"pranav-gandesree".to_vec(),
        };

        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");

        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Transaction failed");
        
        println!("Enrollment complete! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }
}



