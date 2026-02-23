#![no_std]
#![no_main]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

/// Class C LoRaWAN module
pub mod class_c;
/// C FFI Bindings for ASR6601 SDK
pub mod ffi;
/// Peripherals
pub mod peripherals;
/// Serial printing
pub mod print;
/// Interrupts
pub mod tremo_it;

use crate::{
    class_c::app_start,
    peripherals::{
        gpio::{GPIO_PIN_0, GPIO_PIN_1},
        rcc::{
            RCC_OSC_XO32K, RCC_PERIPHERAL_GPIOA, RCC_PERIPHERAL_GPIOB, RCC_PERIPHERAL_GPIOC,
            RCC_PERIPHERAL_GPIOD, RCC_PERIPHERAL_LORA, RCC_PERIPHERAL_PWR, RCC_PERIPHERAL_RTC,
            RCC_PERIPHERAL_SAC, RCC_PERIPHERAL_UART0,
        },
        regs::{GPIOB, RCC, UART0},
    },
};

/// entry point
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    board_init();
    app_start();
}

/// initialize UART for logging
pub fn uart_log_init() {
    let gpio_b = &mut GPIOB.clone();
    gpio_b.set_iomux(GPIO_PIN_0, 1);
    gpio_b.set_iomux(GPIO_PIN_1, 1);

    let uart = &mut UART0.clone();
    uart.init(Default::default()).unwrap();
    uart.cmd(true);
}

/// init board, enable peripheral clocks, etc.
pub fn board_init() {
    let rcc = &mut RCC.clone();
    rcc.enable_oscillator(RCC_OSC_XO32K, true);

    rcc.enable_peripheral_clk(RCC_PERIPHERAL_UART0, true);
    rcc.enable_peripheral_clk(RCC_PERIPHERAL_GPIOA, true);
    rcc.enable_peripheral_clk(RCC_PERIPHERAL_GPIOB, true);
    rcc.enable_peripheral_clk(RCC_PERIPHERAL_GPIOC, true);
    rcc.enable_peripheral_clk(RCC_PERIPHERAL_GPIOD, true);
    rcc.enable_peripheral_clk(RCC_PERIPHERAL_PWR, true);
    rcc.enable_peripheral_clk(RCC_PERIPHERAL_RTC, true);
    rcc.enable_peripheral_clk(RCC_PERIPHERAL_SAC, true);
    rcc.enable_peripheral_clk(RCC_PERIPHERAL_LORA, true);

    unsafe {
        ffi::delay_ms(100);
        ffi::pwr_xo32k_lpm_cmd(true);

        uart_log_init();

        ffi::RtcInit();
    }
}

/// rust panic handler
#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{}:{}",
            location.file(),
            location.line(),
            location.column(),
        );
    } else {
        println!("Panicked at unknown location");
    }
    loop {}
}
