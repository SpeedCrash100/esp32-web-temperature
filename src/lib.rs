#![no_std]
#![feature(impl_trait_in_assoc_type)]

pub mod boards;
pub mod color_temp;
pub mod drivers;
pub mod load_indicator;
pub mod mutex;
pub mod sensor_data;
pub mod web;

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub(crate) use mk_static;
