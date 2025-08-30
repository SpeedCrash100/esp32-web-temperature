#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::rmt::{Rmt, TxChannelConfig, TxChannelCreator};
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::tsens::TemperatureSensor;
use {esp_backtrace as _, esp_println as _};

use esp_temperature::ws2812;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let wifi_init =
        esp_wifi::init(timer1.timer0, rng).expect("Failed to initialize WIFI/BLE controller");
    let (mut _wifi_controller, _interfaces) = esp_wifi::wifi::new(&wifi_init, peripherals.WIFI)
        .expect("Failed to initialize WIFI controller");

    let freq = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, freq)
        .expect("failed to init RMT")
        .into_async();
    let tx_rmt_cfg = TxChannelConfig::default()
        .with_clk_divider(2)
        .with_idle_output(true)
        .with_idle_output_level(Level::Low);
    let tx_rmt_chan = rmt
        .channel0
        .configure_tx(peripherals.GPIO8, tx_rmt_cfg)
        .expect("failed to configure channel for RGB");

    let mut rgb_led = ws2812::init(tx_rmt_chan, freq / 2);
    let temperature_sensor =
        TemperatureSensor::new(peripherals.TSENS, esp_hal::tsens::Config::default())
            .expect("failed to init temp sensor");

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        let temp = temperature_sensor.get_temperature();
        info!("Temperature: {}°C", temp.to_celsius());

        let color = esp_temperature::color_temp::nearest_color(temp.to_celsius() as u16 * 100);

        rgb_led.set_color(color.r(), color.g(), color.b()).await;

        Timer::after(Duration::from_millis(1000)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
