#![warn(rust_2018_idioms)]

extern crate dpdk_sys;
extern crate thiserror;

mod ffi;

pub mod eal;

pub mod prelude {
    pub use eal::EalGlobalApi;
}