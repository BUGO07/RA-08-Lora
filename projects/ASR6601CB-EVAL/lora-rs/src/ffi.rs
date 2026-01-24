#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub static mut UART0: *mut uart_t = UART0_BASE as *mut uart_t;

pub static mut GPIOA: *mut gpio_t = GPIO_BASE as *mut gpio_t;
pub static mut GPIOB: *mut gpio_t = (GPIO_BASE + 0x400) as *mut gpio_t;
pub static mut GPIOC: *mut gpio_t = (GPIO_BASE + 0x800) as *mut gpio_t;
pub static mut GPIOD: *mut gpio_t = (GPIO_BASE + 0xC00) as *mut gpio_t;
