use neo_web3::futures::{future, StreamExt};

#[tokio::main]
async fn main() -> neo_web3::Result {
    let ws = neo_web3::transports::WebSocket::new("ws://localhost:8546").await?;
    let neo_web3 = neo_web3::Web3::new(ws.clone());
    let mut sub = neo_web3.eth_subscribe().subscribe_new_heads().await?;

    println!("Got subscription id: {:?}", sub.id());

    (&mut sub)
        .take(5)
        .for_each(|x| {
            println!("Got: {:?}", x);
            future::ready(())
        })
        .await;

    sub.unsubscribe().await?;

    Ok(())
}
