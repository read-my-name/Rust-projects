//Import library
use {
    dotenv,
    reqwest,
    tokio,
    serde_json::Result,
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_address::BlockchainAddress,
    crate::blockchain_transaction::BlockchainTransaction,
};

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
// async is for async+await syntax
pub async fn send_request(url: &str) -> String{
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", dotenv::var("API_KEY").expect("Could not find key: API_KEY"))
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert payload")
}

pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    // println!("Response: \n{}", response);
    serde_json::from_str(&response).expect("Failed to parse response")
}