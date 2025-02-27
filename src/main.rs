mod commands;
mod models;
mod proto;
mod storage;
mod wallet;
mod errors;

use commands::Command;
use structopt::StructOpt;
use wallet::WalletClient;
use errors::WalletError;

/// Entry point for the blockchain wallet CLI application.
/// 
/// Parses command-line arguments and delegates to the `run` function.
/// Handles any errors that occur during execution and provides appropriate
/// error messages to the user.
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// The main application logic for the blockchain wallet CLI.
/// 
/// Processes the command-line arguments, initializes the wallet client,
/// and executes the requested command.
/// 
/// # Returns
/// 
/// * `Ok(())` - If the command executes successfully
/// * `Err(WalletError)` - If an error occurs during execution
async fn run() -> Result<(), WalletError> {
    let command = Command::from_args();
    let mut wallet = WalletClient::new().await?;

    match command {
        Command::CreateWallet { name } => {
            match wallet.create_wallet(&name) {
                Ok(_) => {
                    let keypair = wallet.get_wallet(&name).unwrap();
                    println!("New wallet '{}' created!", name);
                    println!("Address: {}", keypair.public_key);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        
        Command::ListWallets => {
            let wallets = wallet.list_wallets();
            if wallets.is_empty() {
                println!("No wallets found. Create one with 'new <name>'");
            } else {
                println!("Your wallets:");
                for (name, keypair) in wallets {
                    println!("- {} (address: {})", name, keypair.public_key);
                }
            }
        },
        
        Command::GetBalance { wallet_name } => {
            match wallet.get_balance(&wallet_name).await {
                Ok(balance) => println!("Balance for '{}': {} coins", wallet_name, balance),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        
        Command::SendTransaction { from_wallet, to_wallet, amount } => {
            match wallet.send_transaction(&from_wallet, &to_wallet, amount).await {
                Ok(_) => println!("Transaction sent successfully!"),
                Err(e) => eprintln!("Error sending transaction: {}", e),
            }
        },
        
        Command::RequestFaucet { wallet_name } => {
            match wallet.request_faucet(&wallet_name).await {
                Ok(amount) => println!("Received {} coins to wallet '{}'", amount, wallet_name),
                Err(e) => eprintln!("Error requesting from faucet: {}", e),
            }
        },
    }

    Ok(())
}