use core::future::Future;

pub mod ws2812;

pub trait RgbLedAsync {
    fn set_color(&mut self, r: u8, g: u8, b: u8) -> impl Future<Output = ()>;
}
