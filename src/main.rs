use reqwest; // https://docs.rs/reqwest/latest/reqwest/
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let request = reqwest::Client::new();
    let request_api_value = request
        .get("https://api.chucknorris.io/jokes/gbKrfHdGSCOEA_0HwtW4aA")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
        println!("{:?}", request_api_value);
        Ok(())
}