mod inky;

use crate::inky::Inky;

#[tokio::main]
async fn main(){
    let mut inky = Inky::new();
    inky.setup().await;
    
    let path = std::path::Path::new("input.jpg");
    inky.set_image(path, true);

    inky.show().await;
}
