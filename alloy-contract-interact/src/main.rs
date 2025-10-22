use alloy::{
    primitives::{
        U256, address,
        utils::{Unit, format_ether},
    },
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use std::error::Error;

sol! {
    /// Binding for WETH9 contract
    #[sol(rpc)]
    contract WETH9 {
        function deposit() public payable;
        function balanceOf(address)public view returns (uint256);
        function withdraw(uint256) public;
        function approve(address, uint256) public;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initalizer a signer with a private key (Anvil2)
    let anvil2: PrivateKeySigner =
        "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a".parse()?;

    // Instantiate the provider with the signer
    let provider = ProviderBuilder::new()
        .wallet(anvil2.clone()) // Clone signer to avoid ownership issues later
        .connect_anvil_with_config(|a| a.fork("https://reth-ethereum.ithaca.xyz/rpc"));

    // Instantiate the WETH contract at its known addres
    let weth_address = address!("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
    let weth = WETH9::new(weth_address, provider.clone());

    // Read the initial WETH balance
    let from_address = anvil2.address();
    let initial_balance = weth.balanceOf(from_address).call().await?;
    println!("🥶 Intial WETH Balance: {}", format_ether(initial_balance));

    // Deposit ETH to receive WETH
    let deposit_amount = Unit::ETHER.wei().saturating_mul(U256::from(20));
    let deposit_tx = weth.deposit().value(deposit_amount).send().await?;
    let deposit_receipt = deposit_tx.get_receipt().await?;
    println!(
        "✅ Deposited {} ETH in block {}",
        format_ether(deposit_amount),
        deposit_receipt
            .block_number
            .expect("Failed to get block value.")
    );

    // Read the updated WETH balance
    let updated_balance = weth.balanceOf(from_address).call().await?;
    println!(
        "💰 Updated WETH Balance: {} WETH",
        format_ether(updated_balance)
    );

    // Approve WETH spending
    let approve_amount = Unit::ETHER.wei().saturating_mul(U256::from(10));
    let approve_tx = weth.approve(from_address, approve_amount).send().await?;
    let approve_receipt = approve_tx.get_receipt().await?;
    println!(
        "👌 Approved {} WETH in block {}",
        format_ether(approve_amount),
        approve_receipt
            .block_number
            .expect("Failed to get block value.")
    );

    // Withdraw WETH to receive ETH back
    // NOT WORKING DUE TO FORKED NETWORK RESTRICTIONS SOMEHOW
    // let withdraw_amount = Unit::ETHER.wei().saturating_mul(U256::from(1));
    // let withdraw_tx = weth.withdraw(withdraw_amount).send().await?;
    // let withdraw_receipt = withdraw_tx.get_receipt().await?;
    // println!(
    //     "✅ Withdrew {} WETH in block {}",
    //     format_ether(withdraw_amount),
    //     withdraw_receipt
    //         .block_number
    //         .expect("Failed to get block value.")
    // );

    // Read the final WETH balance
    let final_balance = weth.balanceOf(from_address).call().await?;
    println!(
        "🏁 Final WETH Balance: {} WETH",
        format_ether(final_balance)
    );

    Ok(())
}
