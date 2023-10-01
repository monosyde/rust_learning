use serde::{Deserialize, Serialize};
use std::env;

use crate::model::Card;

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
