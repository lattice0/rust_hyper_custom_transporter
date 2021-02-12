mod custom_req;
use custom_req::CustomTransporter;

use std::env;

use hyper::{body::Body, Client};
use tokio::io::{self, AsyncWriteExt as _};
use hyper::client::connect::HttpConnector;
use hyper::body::HttpBody;


// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    //let client = Client::new();
    let connector = CustomTransporter::new();
    //let client = Client::<(), Body>::builder().build(connector);
    println!("creating client");
    let client: Client<CustomTransporter, hyper::Body> = Client::builder().build(connector);
    println!("did create client");

    println!("fetching url: {}", url.host().unwrap());

    let mut res = client.get(url).await.unwrap();
    
    println!("passed get");
    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}
