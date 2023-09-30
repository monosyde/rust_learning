use dotenv::dotenv;
use serde_json::{self};
use std::env;

mod kaiten;
mod model;
mod utils;

fn main() {
    dotenv().ok();

    let url_get_cards = env::var("URL_GET_CARDS").unwrap();

    let cards_response = kaiten::get(&url_get_cards);
    let cards_text = &cards_response.text().unwrap();
    let cards: Vec<model::Card> =
        serde_json::from_str(cards_text).unwrap();

    let filtered_cards = utils::filter_cards_by_member_id(&cards);
    // println!("filtered_cards {:#?}", &filtered_cards);
    let _titles = utils::get_titles_from_cards(&filtered_cards);
    // println!("titles {:#?}", &titles);
    let info = utils::get_info_from_cards(&filtered_cards);
    println!("info {:#?}", &info);
}
