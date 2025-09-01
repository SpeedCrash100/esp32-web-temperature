use esp_hal::{
    gpio::Level,
    rmt::{TxChannelConfig, TxChannelCreator},
    time::Rate,
    Async,
};

type RgbLedChannelCreator = esp_hal::rmt::ChannelCreator<Async, 0>;
type RgbLedChannel =
    esp_hal::rmt::Channel<Async, <RgbLedChannelCreator as TxChannelCreator<'static, Async>>::Raw>;

pub type RgbLed = crate::drivers::led::ws2812::WS2812<RgbLedChannel>;

pub async fn init_rgb_led(
    rmt_ch: RgbLedChannelCreator,
    input_freq: Rate,
    gpio: esp_hal::gpio::AnyPin<'static>,
) -> RgbLed {
    const DIV: u8 = 2;

    let tx_rmt_cfg = TxChannelConfig::default()
        .with_clk_divider(DIV)
        .with_idle_output(true)
        .with_idle_output_level(Level::Low);
    let tx_rmt_chan = rmt_ch
        .configure_tx(gpio, tx_rmt_cfg)
        .expect("failed to configure channel for RGB");

    let rgb_led: RgbLed = crate::drivers::led::ws2812::init(tx_rmt_chan, input_freq / (DIV as u32));

    rgb_led
}
