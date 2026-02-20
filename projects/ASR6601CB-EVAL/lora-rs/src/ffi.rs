#![allow(warnings)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub static mut GPIOA: *mut gpio_t = GPIO_BASE as *mut gpio_t;
pub static mut GPIOB: *mut gpio_t = (GPIO_BASE + 0x400) as *mut gpio_t;
pub static mut GPIOC: *mut gpio_t = (GPIO_BASE + 0x800) as *mut gpio_t;
pub static mut GPIOD: *mut gpio_t = (GPIO_BASE + 0xC00) as *mut gpio_t;
