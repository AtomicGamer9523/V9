#![no_std]
#![warn(clippy::all)]
#![feature(naked_functions)]

pub mod thread;
pub use embassy_time as time;