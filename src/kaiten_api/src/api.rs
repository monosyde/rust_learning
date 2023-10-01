use reqwest::{header, Response};
use dotenv::dotenv;
use std::env;

pub async fn get(url: &str) -> Response {
    let client = reqwest::Client::new();

    let bearer = env::var("HEADER_BEARER").unwrap();
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&bearer).unwrap(),
    );

    client.get(url).headers(headers).send().await.unwrap()
}

async fn perform_get_request() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let url_get_cards = env::var("URL_GET_CARDS").unwrap();
    let response = get(&url_get_cards).await;

    println!("resp {:?}", response.status());

    assert!(response.status().is_success());

    Ok(())
}

#[tokio::test]
async fn test_get_request() {
    match perform_get_request().await {
        Ok(_) => println!("Test passed"),
        Err(e) => eprintln!("Test failed: {:?}", e),
    }
}
