mod inky;
use rppal::gpio::Gpio;
use std::{thread, time::Duration};

use crate::inky::{Inky, HEIGHT, WIDTH};

#[tokio::main]
async fn main(){
    // Pin 18 corresponds to BCM GPIO 18
    let mut inky = Inky::new();
    inky.setup_2().await;
    for y in 0..(HEIGHT - 1) {
        let c = y / (HEIGHT / 6);
        for x in 0..(WIDTH - 1) {
            inky.set_pixel(x, y, c as u8);
        }
    }
    inky.update().await;

    // or y in range(inky.height - 1):
    // c = y // (inky.height // 6)
    // for x in range(inky.width - 1):
    //     inky.set_pixel(x, y, COLOURS[c])

}
