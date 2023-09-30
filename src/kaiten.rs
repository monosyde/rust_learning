use std::env;
use reqwest::{header, blocking};


pub fn kaiten_get_by_url(url: &String) -> blocking::Response {
    let client = reqwest::blocking::Client::new();

    let bearer = env::var("HEADER_BEARER").unwrap();
    let mut headers = header::HeaderMap::new();    

    headers
        .insert(header::AUTHORIZATION, header::HeaderValue::from_str(&bearer)
        .unwrap());

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .unwrap();

    response
}