use alloy::{
    network::TransactionBuilder,
    primitives::{
        U256, address,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init signer with private key
    let signer: PrivateKeySigner =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;

    // Init provider with the signer and local anvil node
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect("http://localhost:8545")
        .await?;

    // Build tx to send 10 ETH to anvil 10
    let anvil_10 = address!("0xa0Ee7A142d267C1f36714E4a8F75612F20a79720");
    // Performs multiplication that won't panic on overflow to get 10 ETH in Wei
    let value = Unit::ETHER.wei().saturating_mul(U256::from(10));
    let tx = TransactionRequest::default()
        .with_to(anvil_10)
        .with_value(value);

    // Send the tx and wait for the receipt
    let pendind_tx = provider.send_transaction(tx).await?;
    println!("🧰 Pending tx... {}", pendind_tx.tx_hash());

    // Wait for tx to be included in a block
    let receipt = pendind_tx.get_receipt().await?;

    println!(
        "✅ Tx minded in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    println!("💵 Transfered {:.5} ETH to {anvil_10}", format_ether(value));

    Ok(())
}
