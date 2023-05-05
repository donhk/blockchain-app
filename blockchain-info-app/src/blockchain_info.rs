use dotenv;
use reqwest;
use tokio;
use serde::Result;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

pub fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client.get(url).header("api-key", "");
}