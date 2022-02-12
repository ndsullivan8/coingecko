use futures::future::join_all;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Coin {
    ethereum: Ethereum,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Ethereum {
    usd: f64,
}

async fn fetch(url: &str) -> Result<Coin, reqwest::Error> {
    let resp = reqwest::get(url)
        .await?
        .json::<Coin>()
        .await?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let eth_url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}", "ethereum", "usd");
    let bad_url = "https://www.foo.com".to_string();
    let mut futures = Vec::new();
    for i in 1..5 {
        if i % 5 == 0 {
            futures.push(fetch(&bad_url));
        } else {
            futures.push(fetch(&eth_url));
        }
    }
    let resps = join_all(futures).await;
    resps.into_iter().for_each(|resp| {
        println!(
            "{:#?}",
            resp.unwrap()
        );
    });
    Ok(())
}
