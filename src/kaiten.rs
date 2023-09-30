use reqwest::{blocking, header};
use std::env;

pub fn get(url: &String) -> blocking::Response {
    let client = reqwest::blocking::Client::new();

    let bearer = env::var("HEADER_BEARER").unwrap();
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&bearer).unwrap(),
    );

    client.get(url).headers(headers).send().unwrap()
}
