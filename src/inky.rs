use std::time::Duration;

use rppal::{gpio::Gpio, spi::{Bus, Mode, SlaveSelect, Spi}};
use tokio::time::Instant;

enum Colours {
    BLACK = 0,
    WHITE = 1,
    YELLOW = 3,
    RED = 4,
    BLUE = 5,
    GREEN = 6
}

const RESET_PIN: u8 = 27;
const BUSY_PIN: u8 = 17;
const DC_PIN: u8 = 22;

const MOSI_PIN: u8 = 10;
const SCLK_PIN: u8 = 11;
const CS0_PIN: u8 = 8;

const EL673_PSR: u8 = 0x00;
const EL673_PWR: u8 = 0x01;
const EL673_POF: u8 = 0x02;
const EL673_POFS: u8 = 0x03;
const EL673_PON: u8 = 0x04;
const EL673_BTST1: u8 = 0x05;
const EL673_BTST2: u8 = 0x06;
const EL673_DSLP: u8 = 0x07;
const EL673_BTST3: u8 = 0x08;
const EL673_DTM1: u8 = 0x10;
const EL673_DSP: u8 = 0x11;
const EL673_DRF: u8 = 0x12;
const EL673_PLL: u8 = 0x30;
const EL673_CDI: u8= 0x50;
const EL673_TCON: u8 = 0x60;
const EL673_TRES: u8 = 0x61;
const EL673_REV: u8 = 0x70;
const EL673_VDCS: u8 = 0x82;
const EL673_PWS: u8= 0xE3;

const MAX_SPEED_HZ: u32 = 1_000_000;

pub const RESOLUTION: (i32, i32) = (800, 480);
pub const WIDTH: usize = RESOLUTION.0 as usize;
pub const HEIGHT: usize = RESOLUTION.1 as usize;

pub struct Inky {
    reset_pin: rppal::gpio::OutputPin,
    dc_pin: rppal::gpio::OutputPin,
    cs_pin: rppal::gpio::OutputPin,
    busy_pin: rppal::gpio::InputPin,
    spi: Spi,
    buf: Vec<u8>
}

impl Inky {
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

    pub async fn setup_2(&mut self) {
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

    async fn send_command(&mut self, command: u8, data: Option<Vec<u8>>) {
        self.cs_pin.set_low();
        self.dc_pin.set_low();
        tokio::time::sleep(Duration::from_millis(300)).await;
        self.spi.write(&[command]).unwrap();
        
        // if data is not none
        if let Some(data) = data {
            self.dc_pin.set_high();
            
            // Split large data into chunks to avoid "Message too long" error
            const CHUNK_SIZE: usize = 4096; // 4KB chunks
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
            .chunks(2)                       // take two elements at a time
            .map(|pair| {
                let high = (pair[0] << 4) & 0xF0;
                let low = if pair.len() > 1 { pair[1] & 0x0F } else { 0 };
                high | low
            })
            .collect()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, v: u8) {
        self.buf[y*WIDTH+x] = v & 0x07;
    }

    pub async fn show(&mut self) {
        println!("Showing image");
        self.update().await;
    }

    pub async fn update(&mut self) {
        println!("Setting up display");
        self.setup_2().await;

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
}