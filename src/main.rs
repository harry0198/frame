mod inky;

use crate::inky::{Colours, Inky, HEIGHT, WIDTH};

#[tokio::main]
async fn main(){
    // Pin 18 corresponds to BCM GPIO 18
    let mut inky = Inky::new();
    inky.setup().await;
    
    let path = std::path::Path::new("input.jpg");
    inky.set_image(path);

    inky.show().await;

    // or y in range(inky.height - 1):
    // c = y // (inky.height // 6)
    // for x in range(inky.width - 1):
    //     inky.set_pixel(x, y, COLOURS[c])

}
