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

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;

use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::interrupt::Priority;
use esp_hal::rmt::Rmt;
use esp_hal::rng::Rng;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;

use esp_hal::timer::timg::TimerGroup;
use esp_hal_embassy::InterruptExecutor;
use esp_temperature::drivers::sensors::dht22::Dht22Esp32;
use esp_temperature::load_indicator::LoadExecutorHook;
use esp_temperature::web::{SharedHumidity, SharedTemp};
use esp_wifi::EspWifiController;

use {esp_backtrace as _, esp_println as _};

use esp_temperature::boards::esp32::esp32_c6::*;
use esp_temperature::drivers::led::RgbLedAsync as _;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

/// Load from main executor
static CPU_LOAD_THREADING: AtomicU8 = AtomicU8::new(100);

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[esp_hal::main]
fn main() -> ! {
    let mut executor = ::esp_hal_embassy::Executor::new();
    // Safety: main never returns means it is always will exists
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

        Timer::after_millis(250).await
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

    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let executor_medium = mk_static!(
        InterruptExecutor<0>,
        InterruptExecutor::new(sw_ints.software_interrupt0)
    );

    let spawner_medium = executor_medium.start(Priority::Priority3);

    info!("Embassy initialized!");

    let freq = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, freq)
        .expect("failed to init RMT")
        .into_async();

    let rgb_led = init_rgb_led(rmt.channel0, freq, peripherals.GPIO8.into()).await;
    spawner.must_spawn(indicate_load(rgb_led));

    let rng = Rng::new(peripherals.RNG);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let esp32_wifi_ctrl = &*mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timg0.timer0, rng).expect("failed to init esp radio ctrl")
    );

    let stack = start_wifi(esp32_wifi_ctrl, peripherals.WIFI, rng, spawner).await;

    let web_temperature = mk_static!(Mutex<CriticalSectionRawMutex, f32>,Mutex::new(0.0_f32));
    let shared_temperature = SharedTemp::new(web_temperature);
    let web_humidity = mk_static!(Mutex<CriticalSectionRawMutex, f32>,Mutex::new(0.0_f32));
    let shared_humidity = SharedHumidity::new(web_humidity);

    let web_app_state = mk_static!(
        esp_temperature::web::AppState,
        esp_temperature::web::AppState {
            temp: shared_temperature.clone(),
            humidity: shared_humidity.clone(),
        }
    );

    let web_app = esp_temperature::web::WebApp::default();
    for id in 0..esp_temperature::web::WEB_TASK_POOL_SIZE {
        spawner.must_spawn(esp_temperature::web::web_task(
            id,
            stack,
            web_app.router,
            web_app.config,
            web_app_state,
        ));
    }

    let dht_pin = esp_hal::gpio::Flex::new(peripherals.GPIO4);

    let dht = Dht22Esp32::new(dht_pin);
    spawner_medium.must_spawn(publish_web_environment(
        dht,
        shared_temperature,
        shared_humidity,
    ));

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

#[embassy_executor::task]
async fn publish_web_environment(
    mut dht: Dht22Esp32,
    out_temp: SharedTemp,
    out_humidity: SharedHumidity,
) {
    loop {
        Timer::after_secs(2).await;

        if embassy_time::with_timeout(Duration::from_millis(1500), dht.read())
            .await
            .is_err()
        {
            error!("failed to get environment data");
            dht.reset().await;
            continue;
        }

        let temp = dht.temperature();
        let humi = dht.humidity();

        out_temp.set(temp).await;
        out_humidity.set(humi).await;
    }
}
