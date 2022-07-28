mod client;
mod endpoints;
mod enums;
mod response;

pub use client::*;
pub use enums::*;
pub use tokio::*;

// set via `.env`
const ETHERSCAN_API_KEY: &str = "";
// set via `.env`
const CONTRACT_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
// set via `.env`
const ADDRESS: &str = "0x0000000000000000000000000000000000000000";
const PAGE: u32 = 1u32;
const OFFSET: u32 = 0u32;
const START_BLOCK: u32 = 10_132_771u32;
const END_BLOCK: u32 = 10_232_771u32;

#[tokio::main]
async fn main() {
    let client_instance: Client = Client::new(ETHERSCAN_API_KEY);

    match Client::transfers_erc20(
        &client_instance,
        CONTRACT_ADDRESS,
        ADDRESS,
        PAGE,
        OFFSET,
        START_BLOCK,
        END_BLOCK,
        enums::Sort::Asc,
    )
    .await
    {
        // Variable `response` can be named anything bc it's a placeholder.
        Ok(response) => {
            println!("The result from API request: {:?}", response);
        }
        // Variable `e` can be named anything bc it's a placeholder, but it
        // is common to use `e` or `error`.
        Err(e) => {
            panic!("there was an error!: {:?}", e);
        }
    }
}
