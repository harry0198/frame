use std::{path::Path, time::Duration};

use image::{imageops::{self, ColorMap, FilterType},Rgba};
use rppal::{gpio::Gpio, spi::{Bus, Mode, SlaveSelect, Spi}};
use tokio::time::Instant;

const RESET_PIN: u8 = 27;
const BUSY_PIN: u8 = 23;
const DC_PIN: u8 = 22;

const CS0_PIN: u8 = 8;

const EL673_PSR: u8 = 0x00;
const EL673_PWR: u8 = 0x01;
const EL673_POF: u8 = 0x02;
const EL673_POFS: u8 = 0x03;
const EL673_PON: u8 = 0x04;
const EL673_BTST1: u8 = 0x05;
const EL673_BTST2: u8 = 0x06;
const EL673_BTST3: u8 = 0x08;
const EL673_DTM1: u8 = 0x10;
const EL673_DRF: u8 = 0x12;
const EL673_PLL: u8 = 0x30;
const EL673_CDI: u8= 0x50;
const EL673_TCON: u8 = 0x60;
const EL673_TRES: u8 = 0x61;
const EL673_VDCS: u8 = 0x82;
const EL673_PWS: u8= 0xE3;

const MAX_SPEED_HZ: u32 = 1_000_000;

pub const RESOLUTION: (i32, i32) = (800, 480);
pub const WIDTH: usize = RESOLUTION.0 as usize;
pub const HEIGHT: usize = RESOLUTION.1 as usize;

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Colours {
    BLACK = 0,
    WHITE = 1,
    YELLOW = 2,
    RED = 3,
    BLUE = 5,
    GREEN = 6
}

impl Colours {
    pub fn as_rgba(self) -> Rgba<u8> {
        match self {
            Colours::BLACK  => Rgba([0, 0, 0, 255]),
            Colours::WHITE  => Rgba([255, 255, 255, 255]),
            Colours::YELLOW => Rgba([255, 242, 0, 255]),
            Colours::RED    => Rgba([255, 0, 0, 255]),
            Colours::BLUE   => Rgba([0, 0, 217, 255]),
            Colours::GREEN  => Rgba([0, 228, 0, 255]),
        }
    }
}

struct Palette;

impl ColorMap for Palette {
    type Color = Rgba<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        let colors = [
            Colours::BLACK.as_rgba(),
            Colours::WHITE.as_rgba(),
            Colours::YELLOW.as_rgba(),
            Colours::RED.as_rgba(),
            Colours::BLUE.as_rgba(),
            Colours::GREEN.as_rgba(),
        ];
        
        let mut min_distance = f32::MAX;
        let mut best_index = 0;
        
        for (i, palette_color) in colors.iter().enumerate() {
            let distance = color_distance(color, palette_color);
            if distance < min_distance {
                min_distance = distance;
                best_index = match i {
                    0 => Colours::BLACK as usize,
                    1 => Colours::WHITE as usize,
                    2 => Colours::YELLOW as usize,
                    3 => Colours::RED as usize,
                    4 => Colours::BLUE as usize,
                    5 => Colours::GREEN as usize,
                    _ => Colours::BLACK as usize,
                };
            }
        }
        
        best_index
    }

    fn map_color(&self, color: &mut Self::Color) {
        let idx = self.index_of(color);
        *color = self.lookup(idx).unwrap_or(Colours::BLACK.as_rgba());
    }
    
    fn lookup(&self, index: usize) -> Option<Self::Color> {
        let color = match index {
            0 => Colours::BLACK.as_rgba(),
            1 => Colours::WHITE.as_rgba(),
            2 => Colours::YELLOW.as_rgba(),
            3 => Colours::RED.as_rgba(),
            5 => Colours::BLUE.as_rgba(),
            6 => Colours::GREEN.as_rgba(),
            _ => Colours::BLACK.as_rgba()
        };
        Some(color)
    }
    
    fn has_lookup(&self) -> bool {
        true
    }
}

pub struct Inky {
    reset_pin: rppal::gpio::OutputPin,
    dc_pin: rppal::gpio::OutputPin,
    cs_pin: rppal::gpio::OutputPin,
    busy_pin: rppal::gpio::InputPin,
    spi: Spi,
    buf: Vec<u8>
}

impl Inky {
    /*
    Initialize the new Inky object with the necessary GPIO pins and SPI interface.
     */
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        let reset_pin = gpio.get(RESET_PIN).unwrap().into_output();
        let dc_pin = gpio.get(DC_PIN).unwrap().into_output();
        let busy_pin = gpio.get(BUSY_PIN).unwrap().into_input_pullup();
        let cs_pin = gpio.get(CS0_PIN).unwrap().into_output();

        // CS0_PIN 8 = Ss0
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, MAX_SPEED_HZ, Mode::Mode0).unwrap();
        let buf = vec![0; WIDTH * HEIGHT];

        Inky {
            reset_pin,
            dc_pin,
            cs_pin,
            busy_pin,
            spi,
            buf
        }
    }

    pub async fn setup(&mut self) {
        self.reset_pin.set_low();
        tokio::time::sleep(Duration::from_millis(30)).await;
        self.reset_pin.set_high();
        tokio::time::sleep(Duration::from_millis(30)).await;

        self.busy_wait(Duration::from_millis(300)).await;

        self.send_command(0xAA, Some(vec![0x49, 0x55, 0x20, 0x08, 0x09, 0x18])).await;
        self.send_command(EL673_PWR, Some(vec![0x3F])).await;
        self.send_command(EL673_PSR, Some(vec![0x5F, 0x69])).await;

        self.send_command(EL673_BTST1, Some(vec![0x40, 0x1F, 0x1F, 0x2C])).await;
        self.send_command(EL673_BTST3, Some(vec![0x6F, 0x1F, 0x1F, 0x22])).await;
        self.send_command(EL673_BTST2, Some(vec![0x6F, 0x1F, 0x17, 0x17])).await;

        self.send_command(EL673_POFS, Some(vec![0x00, 0x54, 0x00, 0x44])).await;
        self.send_command(EL673_TCON, Some(vec![0x02, 0x00])).await;
        self.send_command(EL673_PLL, Some(vec![0x08])).await;
        self.send_command(EL673_CDI, Some(vec![0x3F])).await;
        self.send_command(EL673_TRES, Some(vec![0x03, 0x20, 0x01, 0xE0])).await;
        self.send_command(EL673_PWS, Some(vec![0x2F])).await;
        self.send_command(EL673_VDCS, Some(vec![0x01])).await;
    }

    /*
    Sets a pixel in the buffer to a given color value (0-6).
     */
    pub fn set_pixel(&mut self, x: usize, y: usize, v: u8) {
        self.buf[y*WIDTH+x] = v & 0x07;
    }
    

    /* Sets the entire canvas as an image. Forces size to be WIDTH & HEIGHT.
        Provides options for dithering using Floyd-Steinberg algorithm which visually improves
        the appearance of images with limited color palettes.
     */
    pub fn set_image(&mut self, path: &Path, dither: bool) {
        let img = image::open(path).expect("Failed to open image");
        let img = img.resize_exact(WIDTH as u32, HEIGHT as u32, FilterType::Lanczos3);
        let mut img_buf = img.to_rgba8();
        let cmap = Palette;
        
        if dither {
            imageops::dither(&mut img_buf, &cmap);
        }

        let (img_width, img_height) = img_buf.dimensions();

        for y in 0..img_height {
            for x in 0..img_width {
                let pixel = img_buf.get_pixel(x, y);
                let colour = cmap.index_of(pixel) as u8;
                self.set_pixel(x as usize, y as usize, colour);
            }
        }
    }

    /*
    Sends the current buffer to the Inky display and updates the screen.
     */
    pub async fn show(&mut self) {
        self.setup().await;

        let packed_buf = Inky::pack_nibbles(&self.buf);

        self.send_command(EL673_DTM1, Some(packed_buf)).await;
        self.send_command(EL673_PON, None).await;
        self.busy_wait(Duration::from_millis(300)).await;
        self.send_command(EL673_BTST2, Some(vec![0x6F, 0x1F, 0x17, 0x49])).await;

        self.send_command(EL673_DRF, Some(vec!(0x00))).await;

        self.busy_wait(Duration::from_secs(32)).await;

        self.send_command(EL673_POF, Some(vec!(0x00))).await;

        self.busy_wait(Duration::from_secs(32)).await;
    }

    /*
    Reads the busy input pin and waits until it goes high or the timeout is reached.
     */
    async fn busy_wait(&self, timeout: Duration) {
        // if the busy pin is high, assume we're not getting a signal from
        // inky and wait the timeout period to be safe.
        if self.busy_pin.is_high() {
            tokio::time::sleep(timeout).await;
            return;
        }

        let now = Instant::now();
        while !self.busy_pin.is_high() {
            tokio::time::sleep(Duration::from_millis(100)).await;
            if now.elapsed() > timeout {
                panic!("Timeout waiting for busy pin");
            }
        }
    }

    /*
    Sends a given command to the Inky display, optionally with data.
    SPI max transfer size is usually 4096 bytes, so data is sent in chunks.
     */
    async fn send_command(&mut self, command: u8, data: Option<Vec<u8>>) {
        self.cs_pin.set_low();
        self.dc_pin.set_low();
        tokio::time::sleep(Duration::from_millis(300)).await;
        self.spi.write(&[command]).unwrap();
        
        // if data is not none
        if let Some(data) = data {
            self.dc_pin.set_high();
            
            // Split large data into chunks to avoid "Message too long" error
            const CHUNK_SIZE: usize = 4096;
            for chunk in data.chunks(CHUNK_SIZE) {
                
                match self.spi.write(chunk) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Error writing data chunk: {:?}", e);
                        break;
                    }
                }
            }
        }

        self.cs_pin.set_high();
        self.dc_pin.set_low();
    }

    fn pack_nibbles(input: &[u8]) -> Vec<u8> {
        input
            .chunks(2)
            .map(|pair| {
                let high = (pair[0] << 4) & 0xF0;
                let low = if pair.len() > 1 { pair[1] & 0x0F } else { 0 };
                high | low
            })
            .collect()
    }
}

// Calculate the distance between two colors in RGB space.
fn color_distance(c1: &Rgba<u8>, c2: &Rgba<u8>) -> f32 {
    let r = c1.0[0] as f32 - c2.0[0] as f32;
    let g = c1.0[1] as f32 - c2.0[1] as f32;
    let b = c1.0[2] as f32 - c2.0[2] as f32;
    (r * r + g * g + b * b).sqrt()
}
