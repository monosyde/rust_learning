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

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn test_get() {
        let server = MockServer::start();

        let _ = server.mock(|when, then| {
            when.method(GET).path("/example");
            then.status(200).body("Hello, World!");
        });

        env::set_var("HEADER_BEARER", "your_bearer_token");

        let response = get(&format!("{}/example", server.base_url()));
        assert!(response.status().is_success());

        let body = response.text().unwrap();
        assert_eq!(body, "Hello, World!");
    }
}
