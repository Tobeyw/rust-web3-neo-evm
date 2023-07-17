/*
 * @Descripttion: 
 * @version: 
 * @Author: Mindy
 * @Date: 2023-07-17 11:07:12
 */
#[tokio::main]
async fn main() -> neo_web3::Result {
    let _ = env_logger::try_init();
    let http = neo_web3::transports::Http::new("http://localhost:8545")?;
    let neo_web3 = neo_web3::Web3::new(neo_web3::transports::Batch::new(http));

    let accounts = neo_web3.eth().accounts();
    let block = neo_web3.eth().block_number();

    let result = neo_web3.transport().submit_batch().await?;
    println!("Result: {:?}", result);

    let accounts = accounts.await?;
    println!("Accounts: {:?}", accounts);

    let block = block.await?;
    println!("Block: {:?}", block);

    Ok(())
}
