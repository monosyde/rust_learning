extern crate kaiten_api;
use tokio;

#[tokio::main]
async fn main() {
    kaiten_api::utils::get_info().await;
}
