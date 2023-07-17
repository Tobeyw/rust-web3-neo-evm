/*
 * @Descripttion: 
 * @version: 
 * @Author: Mindy
 * @Date: 2023-07-17 11:07:12
 */
use hex_literal::hex;
use neo_web3::{
    contract::{Contract, Options},
    types::U256,
};

#[tokio::main]
async fn main() -> neo_web3::contract::Result<()> {
    let _ = env_logger::try_init();
    let http = neo_web3::transports::Http::new("http://localhost:8545")?;
    let neo_web3 = neo_web3::Web3::new(http);

    let my_account = hex!("d028d24f16a8893bd078259d413372ac01580769").into();
    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("./res/contract_token.code").trim_end();
    // Deploying a contract
    let contract = Contract::deploy(neo_web3.eth(), include_bytes!("../src/contract/res/token.json"))?
        .confirmations(0)
        .options(Options::with(|opt| {
            opt.value = Some(5.into());
            opt.gas_price = Some(5.into());
            opt.gas = Some(3_000_000.into());
        }))
        .execute(
            bytecode,
            (U256::from(1_000_000_u64), "My Token".to_owned(), 3u64, "MT".to_owned()),
            my_account,
        )
        .await?;

    let result = contract.query("balanceOf", (my_account,), None, Options::default(), None);
    // Make sure to specify the expected return type, to prevent ambiguous compiler
    // errors about `Detokenize` missing for `()`.
    let balance_of: U256 = result.await?;
    assert_eq!(balance_of, 1_000_000.into());

    // Accessing existing contract
    let contract_address = contract.address();
    let contract = Contract::from_json(
        neo_web3.eth(),
        contract_address,
        include_bytes!("../src/contract/res/token.json"),
    )?;

    let result = contract.query("balanceOf", (my_account,), None, Options::default(), None);
    let balance_of: U256 = result.await?;
    assert_eq!(balance_of, 1_000_000.into());

    Ok(())
}
