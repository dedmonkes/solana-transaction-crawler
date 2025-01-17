//! # Solana Transaction Crawler Library
//! This library is used to crawl a series of transactions for a given account address and extract the data from it.
//! It operates by allowing you to build a Crawler object and then compose a series of filters
//! for transactions and instructions onto it.
//!
//! Using this library requires access to a Solana RPC node with full historical data, otherwise the crawling will only
//! work with very new data.
//!
//! ## Recommended Client Dependencies
//! ```
//! [dependencies]
//! anyhow = "1.0.58"
//! solana-client = "~1.9.29"
//! solana-program = "~1.9.29"
//! tokio = "~1.14.1"
//! ```
//!
//! ## Examples
//!
//! Construct filters to get mints from a candy machine.
//!
//! ```
//! use anyhow::Result;
//! use solana_client::rpc_client::RpcClient;
//! use solana_transaction_crawler::{
//!     crawler::{Crawler, IxAccount},
//!     filters::{IxNumberAccounts, SuccessfulTxFilter, TxHasProgramId},
//! };
//! use solana_program::pubkey;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Set up Solana RPC client.
//!     let client = RpcClient::new("https://rpc.ankr.com/solana");
//!
//!     // The pubkey of the account you want to crawl for transactions. In this example we use the DeGods CMV1 pubkey.
//!     let candy_machine_id = pubkey!("9MynErYQ5Qi6obp4YwwdoDmXkZ1hYVtPUqYmJJ3rZ9Kn");
//!
//!     // Create the Crawler object.
//!     let mut crawler = Crawler::new(client, candy_machine_id);
//!
//!     // Define filters.
//!
//!     // We only want to crawl transactions that have the program id of the Candy Machine V1 program.
//!     let has_program_id = TxHasProgramId::new("cndyAnrLdpjq1Ssp1z8xxDsB8dxe7u4HL5Nxi2K5WXZ");
//!
//!     // This filter gives us only successful transactions, filtering out any with errors.
//!     let successful_tx = SuccessfulTxFilter;
//!
//!     // We know the mintNFT instruction has exactly 14 accounts and is the only instruction on that program with that number of accounts.
//!     let ix_num_accounts = IxNumberAccounts::EqualTo(14);
//!
//!     // Specify accounts we want to retrieve from the transaction.
//!     // 'Unparsed' means we have to specify the actual index of the account in the instruction.
//!     // The mint account is the sixth account in the instruction.
//!     let mint_account = IxAccount::unparsed("mint", 5);
//!
//!     // Add filters to Crawler
//!     crawler
//!         .add_tx_filter(has_program_id)
//!         .add_tx_filter(successful_tx)
//!         .add_ix_filter(ix_num_accounts)
//!         .add_account_index(mint_account);
//!
//!     // Run the Crawler.
//!     let crawled_accounts = crawler.run().await?;
//!
//!     // We labeled our account "mint" so we look it up by the label.
//!     let mint_addresses = &crawled_accounts["mint"];
//!     println!("Items found: {:?}", mint_addresses.len());
//!
//!     Ok(())
//! }
//!```
//! For common use cases, there are pre-built methods that can be used. For example,
//! to get all the mints from a Candy Machine V2 instance:
//!
//! ```
//! use anyhow::Result;
//! use solana_client::rpc_client::RpcClient;
//! use solana_transaction_crawler::crawler::Crawler;
//! use solana_program::pubkey;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Set up Solana RPC client.
//!     let client = RpcClient::new("https://rpc.ankr.com/solana");
//!
//!     // The pubkey of the candy machine v2 account you want to crawl for transactions.
//!     let candy_machine_id = pubkey!("CMaBroFssWYUXxxwmASEQXF9RtBiyq3XwA1Qi1ViTTV");
//!
//!     // Create the Crawler object.
//!     let crawled_accounts = Crawler::get_cmv2_mints(client, candy_machine_id).await?;
//!
//!     // This method returns both 'mint' and 'metadata' accounts under the respective labels.
//!     let mint_addresses = &crawled_accounts["mint"];
//!     let metadata_addresses = &crawled_accounts["metadata"];
//!     println!("Mint Items found: {:?}", mint_addresses.len());
//!     println!("Metadata Items found: {:?}", metadata_addresses.len());
//!
//!     Ok(())
//! }
//! ```
//!

pub mod constants;
pub mod crawler;
pub mod errors;
pub mod filters;
