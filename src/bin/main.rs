#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::mem::transmute;
use core::sync::atomic::{AtomicU8, Ordering};

use defmt::{error, info, trace};
use embassy_executor::Spawner;

use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;
use esp_hal::clock::CpuClock;

use esp_hal::gpio::{Level, Output};
use esp_hal::i2c::master::Config;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;

use esp_temperature::load_indicator::LoadExecutorHook;

use {esp_backtrace as _, esp_println as _};

use esp_temperature::boards::esp32::esp32_c6::*;
use esp_temperature::drivers::led::RgbLedAsync as _;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

/// Load from main executor
static CPU_LOAD_THREADING: AtomicU8 = AtomicU8::new(100);

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

/// Indicates load of CPU by brightness of the RGB Led
#[embassy_executor::task]
async fn indicate_load(mut led: RgbLed) {
    loop {
        let load = CPU_LOAD_THREADING.load(Ordering::SeqCst);

        let load_normalized = (load as u16) * 255 / 100;
        let load_color = (
            load_normalized as u8,
            load_normalized as u8,
            load_normalized as u8,
        );

        trace!("Load {}", load);

        led.set_color(load_color.0, load_color.1, load_color.2)
            .await;

        Timer::after_secs(1).await
    }
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

    let rgb_led = init_rgb_led(rmt.channel0, freq, peripherals.GPIO8.into()).await;
    spawner.must_spawn(indicate_load(rgb_led));

    let _display_reset = Output::new(peripherals.GPIO10, Level::High, Default::default());

    let mut i2c = init_i2c(
        peripherals.I2C0,
        peripherals.GPIO7,
        peripherals.GPIO6,
        spawner,
    )
    .await;

    info!("i2c scan");

    for addr in 1..127 {
        match i2c.write(addr, &[0x00]).await {
            Ok(_) => {
                info!("Found {:x}", addr);
            }
            Err(_) => {
                // error!("{}: {}", addr, err)
            }
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
