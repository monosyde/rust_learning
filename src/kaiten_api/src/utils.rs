use std::env;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::{api, utils, models::Card};


pub fn _print_struct_bytes(text: &String, start: usize) {
    let str_bytes = &text.as_bytes()[start - 20..start + 100];
    let cut_str = String::from_utf8(str_bytes.to_vec()).unwrap();
    println!("cut_str {:?}", cut_str);
}

pub fn filter_cards_by_member_id(
    cards: &Vec<Card>,
    member_id: i64,
) -> Vec<Card> {
    let mut filtered_cards: Vec<Card> = vec![];

    for card in cards {
        if card.members.is_none() {
            continue;
        }

        let members = card.members.clone().unwrap();

        for member in &members {
            if member.id == member_id {
                filtered_cards.push(card.clone())
            }
        }
    }

    filtered_cards
}

pub fn get_titles_from_cards(cards: &Vec<Card>) -> Vec<String> {
    let mut titles: Vec<String> = vec![];

    for card in cards {
        if card.title.is_empty() {
            continue;
        }

        let title = card.title.clone();
        titles.push(title);
    }

    titles
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Info {
    pub title: String,
    pub members: Vec<String>,
}

impl Info {
    pub fn to_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn get_info_from_cards(cards: &Vec<Card>) -> Vec<Info> {
    let mut info_vec: Vec<Info> = vec![];
    for card in cards {
        if card.title.is_empty() {
            continue;
        }

        let card_title = card.title.clone();

        if card.members.is_none() {
            continue;
        }

        let mut members_vec: Vec<String> = vec![];
        let card_members = card.members.clone().unwrap();
        for member in &card_members {
            if member.full_name.is_none() {
                continue;
            }

            let full_name = member.full_name.clone().unwrap();
            members_vec.push(full_name);
        }

        let info_item = Info {
            title: card_title,
            members: members_vec,
        };

        info_vec.push(info_item);
    }

    info_vec
}

pub async fn get_info() -> Vec<Info> {
    dotenv().ok();
    let url_get_cards = env::var("URL_GET_CARDS").unwrap();

    let cards_response = api::get(&url_get_cards).await;
    let cards_text = &cards_response.text().await.unwrap();
    let cards: Vec<Card> =
        serde_json::from_str(cards_text).unwrap();

    let user_id_str =
        env::var("USER_ID").unwrap_or_else(|_| String::from("0"));
    let user_id: i64 = user_id_str.parse().unwrap_or_else(|_| {
        eprintln!("Не удалось преобразовать USER_ID в i64, используется значение по умолчанию 0");
        0
    });

    let filtered_cards =
        utils::filter_cards_by_member_id(&cards, user_id);
    // println!("filtered_cards {:#?}", &filtered_cards);
    let _titles = utils::get_titles_from_cards(&filtered_cards);
    // println!("titles {:#?}", &titles);
    let info = utils::get_info_from_cards(&filtered_cards);
    println!("info {:#?}", &info);

    info
}
