/*
 * @Descripttion: 
 * @version: 
 * @Author: Mindy
 * @Date: 2023-07-17 11:07:12
 */
use hex_literal::hex;

pub type Transport = neo_web3::transports::Either<neo_web3::transports::WebSocket, neo_web3::transports::Http>;

#[tokio::main]
async fn main() -> neo_web3::Result {
    let _ = env_logger::try_init();
    let transport = neo_web3::transports::Http::new("http://localhost:8545")?;

    run(neo_web3::transports::Either::Right(transport)).await
}

async fn run(transport: Transport) -> neo_web3::Result {
    let neo_web3 = neo_web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = neo_web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push(hex!("00a329c0648769a73afac7f9381e08fb43dbea72").into());

    println!("Calling balance.");
    for account in accounts {
        let balance = neo_web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}
