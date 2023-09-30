

use model::Card;
use serde_json::{self};
use reqwest::{self, header};
use dotenv::dotenv;
use std::env;

mod model;
// mod utils;

fn filter_cards_by_member_id(cards: &Vec<Card>) -> Vec<Card> {
    let mut filtered_cards: Vec<Card> = vec![];
    let user_id_str = env::var("USER_ID").unwrap_or_else(|_| String::from("0"));
    let user_id: i64 = user_id_str.parse().unwrap_or_else(|_| {
        eprintln!("Не удалось преобразовать USER_ID в i64, используется значение по умолчанию (0)");
        0
    });

    for card in cards {
        if card.members.is_none() {
            continue;
        }

        let members = card.members.clone().unwrap();

        for member in &members {
            if member.id == user_id {
                filtered_cards.push(card.clone())
            }
        }
    }

    filtered_cards
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
