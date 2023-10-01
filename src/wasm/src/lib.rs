use wasm_bindgen::prelude::*;
use reqwest::Response;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm!");
}

#[wasm_bindgen]
pub async fn get_info() {
    log("get_info");
    let url = "http://localhost:3030/info";
    let client = reqwest::Client::new();
    let cards_response = client.get(url).send().await.unwrap();
    let cards_text = &cards_response.text().await.unwrap();
    // let cards =
    //     serde_json::from_str(cards_text).unwrap();
    log(cards_text);
}
