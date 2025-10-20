mod inky;

use std::env;
use rand::Rng;
use serde::Deserialize;
use reqwest;

use crate::inky::Inky;

#[tokio::main]
async fn main(){

    let image = fetch_image_from_api().await;
    let mut inky = Inky::new();
    inky.setup().await;
    
    let path = std::path::Path::new("input.jpg");
    inky.set_image(path, true);

    inky.show().await;
}

// Pull images from API.
async fn fetch_image_from_api() -> String {
    // TODO: Fall back - if call fails, show error image.
    let api_base_url = env::var("API_BASE_URL").unwrap();
    let api_endpoint = api_base_url + "/images";

    let response = reqwest::get(api_endpoint).await
        .expect("Failed to fetch images")
        .json::<Vec<Image>>()
        .await
        .expect("Failed to parse JSON response");

    let mut rng = rand::rng();

    let random_image_index = rng.random_range(0..response.len());

    response[random_image_index].file_path.clone()
}

#[derive(Deserialize, Debug)]
pub struct Image {
    #[serde(rename = "filePath")]
    pub file_path: String
}