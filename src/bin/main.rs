#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::mem::transmute;
use core::sync::atomic::{AtomicU8, Ordering};

use defmt::{info, println};
use embassy_executor::Spawner;
use embassy_net::{Runner, StackResources};
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::rmt::{ChannelCreator, Rmt, TxChannelConfig, TxChannelCreator};
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::tsens::TemperatureSensor;
use esp_hal::Async;
use esp_temperature::load_indicator::LoadExecutorHook;
use esp_wifi::wifi::{WifiController, WifiDevice, WifiEvent, WifiState};
use esp_wifi::EspWifiController;
use {esp_backtrace as _, esp_println as _};

use esp_temperature::rgb_led::{ws2812, RgbLedAsync};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

/// Load from main executor
static CPU_LOAD_THREADING: AtomicU8 = AtomicU8::new(100);

/// Inits RGB LED
async fn init_rgb_led<const CH: u8>(
    rmt_ch: ChannelCreator<Async, CH>,
    input_freq: Rate,
    gpio: esp_hal::gpio::AnyPin<'static>,
) -> impl RgbLedAsync
where
    ChannelCreator<Async, CH>: TxChannelCreator<'static, Async>,
{
    const DIV: u8 = 2;

    let tx_rmt_cfg = TxChannelConfig::default()
        .with_clk_divider(DIV)
        .with_idle_output(true)
        .with_idle_output_level(Level::Low);
    let tx_rmt_chan = rmt_ch
        .configure_tx(gpio, tx_rmt_cfg)
        .expect("failed to configure channel for RGB");

    let rgb_led = ws2812::init(tx_rmt_chan, input_freq / (DIV as u32));

    rgb_led
}

#[esp_hal::main]
fn main() -> ! {
    let mut executor = ::esp_hal_embassy::Executor::new();
    // Safety:
    let static_executor: &'static mut ::esp_hal_embassy::Executor =
        unsafe { transmute(&mut executor) };

    let hook = LoadExecutorHook::new(&CPU_LOAD_THREADING);

    static_executor.run_with_callbacks(
        |spawner| {
            spawner.must_spawn(self::embassy_main(spawner));
        },
        hook,
    )
}

#[embassy_executor::task]
async fn embassy_main(spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let freq = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, freq)
        .expect("failed to init RMT")
        .into_async();

    let mut rgb_led = init_rgb_led(rmt.channel0, freq, peripherals.GPIO8.into()).await;

    let temperature_sensor =
        TemperatureSensor::new(peripherals.TSENS, esp_hal::tsens::Config::default())
            .expect("failed to init temp sensor");

    loop {
        let temp = temperature_sensor.get_temperature();
        info!("Temperature: {}°C", temp.to_celsius());
        let load = CPU_LOAD_THREADING.load(Ordering::SeqCst);
        info!("Load: {}%", load);

        let color =
            esp_temperature::color_temp::nearest_color((temp.to_celsius() - 20.0) as u16 * 100);

        rgb_led.set_color(color.r(), color.g(), color.b()).await;

        Timer::after(Duration::from_millis(100)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
