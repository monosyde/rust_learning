

use model::Card;
use serde::{Serialize, Deserialize};
use serde_json::{self, Value, Map};
use reqwest::{self, header};
use dotenv::dotenv;
use std::env;

mod model;

fn filter_cards_by_member_id(cards: &Vec<Card>) -> Vec<Card> {
    let mut filtered_cards: Vec<Card> = vec![];

    for card in cards {
        if card.members.is_none() {
            continue;
        }

        let members = card.members.clone().unwrap();

        for member in &members {
            if member.id == 183547 {
                filtered_cards.push(card.clone())
            }
        }
    }

    filtered_cards
}

fn print_struct_bytes(text: &String, start: usize) {
    let str_bytes = &text.as_bytes()[start-20..start+100];
    let cut_str = String::from_utf8(str_bytes.to_vec()).unwrap();
    println!("cut_str {:?}", cut_str);

    // let result_skip = text.chars().skip(1241);
    
    // let mut string_bytes = String::new();
    // for item in result_skip {
    //     string_bytes += &item.to_string();
    //     if string_bytes.len() == 30 {
    //         break;
    //     }
    // }

    // println!("string_bytes {:?}", string_bytes);
}
fn main() {
    dotenv().ok();
    let client = reqwest::blocking::Client::new();

    let bearer = env::var("HEADER_BEARER").unwrap();
    let url_get_cards = env::var("URL_GET_CARDS").unwrap();

    let mut headers = header::HeaderMap::new();
    
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&bearer).unwrap());
    let response = client
        .get(url_get_cards)
        .headers(headers)
        .send().unwrap();
    
    let res_text = &response.text().unwrap();
    let cards: Vec<model::Card> = serde_json::from_str(&res_text).unwrap();

    let filtered_cards = filter_cards_by_member_id(&cards);
    println!("filtered_cards {:#?}", &filtered_cards);

    
}
