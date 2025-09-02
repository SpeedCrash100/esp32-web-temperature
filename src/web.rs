mod routes;

use embassy_net::Stack;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use embassy_time::Duration;
use esp_alloc as _;
use picoserve::{response::File, routing, AppRouter, AppWithStateBuilder, Router};

#[derive(Clone)]
pub struct SharedTemp(&'static Mutex<NoopRawMutex, f32>);

impl SharedTemp {
    pub fn new(m: &'static Mutex<NoopRawMutex, f32>) -> Self {
        Self(m)
    }

    pub async fn get(&self) -> f32 {
        *self.0.lock().await
    }

    pub async fn set(&self, temp: f32) {
        *self.0.lock().await = temp;
    }
}

pub struct AppState {
    pub temp: SharedTemp,
}

impl picoserve::extract::FromRef<AppState> for SharedTemp {
    fn from_ref(state: &AppState) -> Self {
        state.temp.clone()
    }
}

pub struct Application;

impl AppWithStateBuilder for Application {
    type State = AppState;
    type PathRouter = impl routing::PathRouter<AppState>;

    fn build_app(self) -> picoserve::Router<Self::PathRouter, Self::State> {
        picoserve::Router::new()
            .route(
                "/",
                routing::get_service(File::html(include_str!("web/data/index.html"))),
            )
            .route(
                "/index.css",
                routing::get_service(File::css(include_str!("web/data/index.css"))),
            )
            .route(
                "/index.js",
                routing::get_service(File::javascript("web/data/index.js")),
            )
            .route("/temperature", routing::get(routes::get_temperature))
    }
}

pub const WEB_TASK_POOL_SIZE: usize = 2;

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
pub async fn web_task(
    id: usize,
    stack: Stack<'static>,
    router: &'static AppRouter<Application>,
    config: &'static picoserve::Config<Duration>,
    state: &'static AppState,
) -> ! {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve_with_state(
        id,
        router,
        config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
        state,
    )
    .await
}

pub struct WebApp {
    pub router: &'static Router<<Application as AppWithStateBuilder>::PathRouter, AppState>,
    pub config: &'static picoserve::Config<Duration>,
}

impl Default for WebApp {
    fn default() -> Self {
        let router = picoserve::make_static!(AppRouter<Application>, Application.build_app());

        let config = picoserve::make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
                persistent_start_read_request: Some(Duration::from_secs(1))
            })
            .keep_connection_alive()
        );

        Self { router, config }
    }
}
