mod inky;

use std::env::{self, args};
use rand::Rng;
use serde::Deserialize;
use reqwest;

use crate::inky::Inky;

#[tokio::main]
async fn main(){
    let args = args().skip(1).collect::<Vec<_>>();

    let image: String;
    if args.is_empty() {
        image = fetch_image_from_api().await;
    } else {
        image = args[1].clone();
    }
    
    println!("Fetched image path: {}", image);
    let mut inky = Inky::new();
    inky.setup().await;
    
    let path = std::path::Path::new(&image);
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