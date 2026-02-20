#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::macro_metavars_in_unsafe)]

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
    ffi::{GPIO_PIN_0, GPIO_PIN_1},
    peripherals::regs::{GPIOB, RCC, UART0},
};

/// entry point
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    board_init();
    app_start();
}

/// initialize UART for logging
pub fn uart_log_init() {
    unsafe {
        GPIOB.set_iomux(GPIO_PIN_0, 1);
        GPIOB.set_iomux(GPIO_PIN_1, 1);

        UART0.init(Default::default()).unwrap();
        UART0.cmd(true);
    }
}

/// init board, enable peripheral clocks, etc.
pub fn board_init() {
    unsafe {
        RCC.enable_oscillator(ffi::RCC_OSC_XO32K, true);

        RCC.enable_peripheral_clk(ffi::RCC_PERIPHERAL_UART0, true);
        RCC.enable_peripheral_clk(ffi::RCC_PERIPHERAL_GPIOA, true);
        RCC.enable_peripheral_clk(ffi::RCC_PERIPHERAL_GPIOB, true);
        RCC.enable_peripheral_clk(ffi::RCC_PERIPHERAL_GPIOC, true);
        RCC.enable_peripheral_clk(ffi::RCC_PERIPHERAL_GPIOD, true);
        RCC.enable_peripheral_clk(ffi::RCC_PERIPHERAL_PWR, true);
        RCC.enable_peripheral_clk(ffi::RCC_PERIPHERAL_RTC, true);
        RCC.enable_peripheral_clk(ffi::RCC_PERIPHERAL_SAC, true);
        RCC.enable_peripheral_clk(ffi::RCC_PERIPHERAL_LORA, true);

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
