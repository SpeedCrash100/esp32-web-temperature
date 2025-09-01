use defmt::error;
use esp_hal::{
    gpio::Level,
    rmt::{PulseCode, TxChannelAsync},
    time::Rate,
};

use crate::drivers::led::RgbLedAsync;

pub struct WS2812<Chan> {
    chan: Chan,
    rate: Rate,
}

impl<Chan> WS2812<Chan>
where
    Chan: TxChannelAsync,
{
    fn get_length(&self, ns: u32) -> u16 {
        let mhz = self.rate.as_mhz();

        (ns * mhz / 1000) as u16
    }
}

impl<Chan> RgbLedAsync for WS2812<Chan>
where
    Chan: TxChannelAsync,
{
    async fn set_color(&mut self, r: u8, g: u8, b: u8) {
        let t0: u32 = PulseCode::new(
            Level::High,
            self.get_length(350),
            Level::Low,
            self.get_length(800),
        );
        let t1: u32 = PulseCode::new(
            Level::High,
            self.get_length(700),
            Level::Low,
            self.get_length(600),
        );

        let mut data = [0_u32; 25];

        let color: u32 = ((g as u32) << 16) | ((r as u32) << 8) | b as u32;
        for i in (0..24).rev() {
            let p = 2_u32.pow(i);
            let bit = p & color != 0;
            let pulse = if bit { t1 } else { t0 };
            data[23 - i as usize] = pulse;
        }

        data[24] = PulseCode::empty();

        if let Err(err) = self.chan.transmit(&data).await {
            error!("failed to set: {}", err);
        }
    }
}

/// Initializes WS2812 to set one LED color
///
/// # Arguments
/// - `channel` - RMT channel to use
/// - `rate` - base rate of rmt channel input_freq/prescaler_from_tx_cfg
///
pub fn init<C>(channel: C, rate: Rate) -> WS2812<C>
where
    C: TxChannelAsync,
{
    WS2812 {
        chan: channel,
        rate,
    }
}
