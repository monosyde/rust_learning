

use serde::{Serialize, Deserialize};
use serde_json::{self, Value, Map};
use reqwest::{self, header};
use dotenv::dotenv;
use std::env;


fn pretty_print(target: &Value) {
    let pretty_vec = serde_json::to_string_pretty(&target).unwrap();
    println!("pretty_vec = {}", &pretty_vec);
}

fn get_result(result: Option<i64>) -> i64 {
    let res = match result {
        Some(num) => num,
        None => 0
    };

    println!("board id = {}", res);

    res
}

fn main() {
    dotenv().ok();
    let client = reqwest::blocking::Client::new();

    let bearer = env::var("HEADER_BEARER").unwrap();
    let url_get_cards = env::var("URL_GET_BOARDS").unwrap();

    let mut headers = header::HeaderMap::new();
    
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&bearer).unwrap());
    println!("headers = {:?}", &headers);

    let response = client
        .get(url_get_cards)
        .headers(headers)
        .send().unwrap();
    
    let res_text = &response.text().unwrap();
    println!("res_text {}", res_text);
    let des: serde_json::Value = serde_json::from_str(&res_text).unwrap();
    
    let vec_json = des.as_array().unwrap();
    let first_elem = vec_json[0].as_object().unwrap();

    let columns = first_elem.get("columns").unwrap();
    let f_columns = &columns.as_array().unwrap()[0].as_object();

    if f_columns.is_none() {
        return
    };

    let board_id = f_columns.unwrap().get("board_id").unwrap();

    let board_id_type = board_id.as_i64();

    get_result(board_id_type);
    
}
