use defmt::error;
use embassy_time::{Duration, Timer};

use esp_hal::{
    delay::Delay,
    gpio::{Flex, InputConfig, OutputConfig},
};

pub struct Dht22Esp32 {
    pin: Flex<'static>,
    temperature: f32,
    humidity: f32,
    delay: Delay,
}

impl Dht22Esp32 {
    pub fn new(mut pin: Flex<'static>) -> Self {
        let out_config = OutputConfig::default()
            .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain)
            .with_pull(esp_hal::gpio::Pull::Up);
        pin.apply_output_config(&out_config);

        let in_config = InputConfig::default().with_pull(esp_hal::gpio::Pull::Up);
        pin.apply_input_config(&in_config);

        pin.set_output_enable(true);

        Self {
            pin,
            temperature: 0.0,
            humidity: 100.0,
            delay: Delay::new(),
        }
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    pub fn humidity(&self) -> f32 {
        self.humidity
    }

    pub async fn reset(&mut self) {
        self.pin.set_output_enable(true);
    }

    pub async fn read(&mut self) {
        // Start communication: pull pin low for 18ms, then release.
        self.pin.set_output_enable(true);
        self.pin.set_low();
        Timer::after_millis(40).await;
        self.pin.set_high();
        self.pin.set_input_enable(true);
        self.delay.delay_micros(40);

        // Wait for sensor to respond.

        self.wait_for_high();
        self.wait_for_low();

        // // Start reading 40 bits
        let humidity_high = self.read_byte().await;
        let humidity_low = self.read_byte().await;
        let temperature_high = self.read_byte().await;
        let temperature_low = self.read_byte().await;
        let checksum = self.read_byte().await;

        self.pin.set_output_enable(true);

        let sum = humidity_high
            .wrapping_add(humidity_low)
            .wrapping_add(temperature_high)
            .wrapping_add(temperature_low);
        if sum != checksum {
            error!("DHT: checksum mismatched");
        }

        let humidity_value = ((humidity_high as u16) << 8) | (humidity_low as u16);
        let humidity_percentage = humidity_value as f32 / 10.0;

        let temperature_high_clean = temperature_high & 0x7F; // 0x7F = 0111 1111
        let temperature_value = ((temperature_high_clean as u16) << 8) | (temperature_low as u16);
        let mut temperature_percentage = temperature_value as f32 / 10.0;

        embassy_time::block_for(Duration::from_micros(30));
        if temperature_high & 0x80 != 0 {
            temperature_percentage = -temperature_percentage;
        }

        self.temperature = temperature_percentage;
        self.humidity = humidity_percentage;

        // self.pin.set_output_enable(true);
        // self.pin.set_high();
    }

    async fn read_byte(&mut self) -> u8 {
        let mut byte = 0;

        for n in 0..8 {
            self.wait_for_high();
            self.delay.delay_micros(35);
            let is_bit_1 = self.pin.is_high();
            if is_bit_1 {
                let bit_mask = 1 << (7 - (n % 8));
                byte |= bit_mask;
                self.wait_for_low();
            }
        }

        byte
    }

    fn wait_for_high(&mut self) {
        while self.pin.is_low() {
            self.delay.delay_micros(1);
        }
    }

    fn wait_for_low(&mut self) {
        while self.pin.is_high() {
            self.delay.delay_micros(1);
        }
    }
}
