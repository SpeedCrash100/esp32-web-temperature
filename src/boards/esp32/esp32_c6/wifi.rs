use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_net::{Runner, Stack, StackResources};
use embassy_time::{Duration, Timer};
use esp_wifi::{
    wifi::{
        ClientConfiguration, Configuration, ScanConfig, WifiController, WifiDevice, WifiEvent,
        WifiState,
    },
    EspWifiController,
};

use crate::mk_static;

const SSID: Option<&str> = option_env!("SSID");
const PASSWORD: Option<&str> = option_env!("PASSWORD");

pub async fn start_wifi(
    esp_wifi_ctrl: &'static EspWifiController<'static>,
    wifi: esp_hal::peripherals::WIFI<'static>,
    mut rng: esp_hal::rng::Rng,
    spawner: Spawner,
) -> Stack<'static> {
    let (controller, interfaces) = esp_wifi::wifi::new(&esp_wifi_ctrl, wifi).unwrap();
    let wifi_interface = interfaces.sta;
    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    let net_config = embassy_net::Config::dhcpv4(Default::default());

    let (stack, runner) = embassy_net::new(
        wifi_interface,
        net_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        net_seed,
    );

    spawner.must_spawn(net_task(runner));
    spawner.must_spawn(connection(controller));
    spawner.must_spawn(ipv4_watcher(stack));

    stack
}

#[embassy_executor::task]
async fn ipv4_watcher(stack: Stack<'static>) {
    loop {
        stack.wait_config_up().await;
        if let Some(config) = stack.config_v4() {
            info!("Got IP: {}", config.address);
            break;
        }
        stack.wait_config_down().await;
        error!("WiFi config down");
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    info!("start connection task");

    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaConnected => {
                // wait until we're no longer connected

                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let ssid: &'static str = SSID.unwrap_or("example_wifi_ssid").into();
            info!("Trying to connect to {}", ssid);

            let client_config = Configuration::Client({
                let mut config = ClientConfiguration::default();
                config.ssid = ssid.into();
                config.password = PASSWORD.unwrap_or("").into();
                config
            });

            controller
                .set_configuration(&client_config)
                .expect("failed to set WiFi config");
            controller
                .start_async()
                .await
                .expect("failed to start WiFi");

            let scan_config = ScanConfig::default();
            controller
                .scan_with_config_async(scan_config)
                .await
                .expect("failed to scan");
        }

        match controller.connect_async().await {
            Ok(_) => info!("Wifi connected!"),
            Err(e) => {
                info!("Failed to connect to wifi: {:?}", e);
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}
