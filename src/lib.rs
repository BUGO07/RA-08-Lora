#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

/// Class C LoRaWAN module
pub mod class_c;
/// Core Cortex M4 Utilities
pub mod cortex;
/// C FFI Bindings for ASR6601 SDK
pub mod ffi;
/// Interrupts
pub mod interrupts;
/// LoRa module
pub mod lora;
/// LoRa Configuration
pub mod lora_config;
/// Peripherals
pub mod peripherals;
/// Serial printing
pub mod print;

use crate::{
    class_c::app_start,
    peripherals::{
        delay::delay_ms,
        gpio::{GpioMode, GpioPin},
        rcc::{
            RCC_OSC_XO32K, RCC_PERIPHERAL_GPIOA, RCC_PERIPHERAL_GPIOB, RCC_PERIPHERAL_GPIOC,
            RCC_PERIPHERAL_GPIOD, RCC_PERIPHERAL_LORA, RCC_PERIPHERAL_PWR, RCC_PERIPHERAL_RTC,
            RCC_PERIPHERAL_SAC, RCC_PERIPHERAL_UART0,
        },
        regs::{GPIOA, GPIOB, PWR, RCC, UART0},
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
    GPIOB.set_iomux(GpioPin::Pin0, 1);
    GPIOB.set_iomux(GpioPin::Pin1, 1);

    UART0.init(Default::default()).unwrap();
    UART0.cmd(true);
}

/// init board, enable peripheral clocks, etc.
pub fn board_init() {
    RCC.enable_oscillator(RCC_OSC_XO32K, true);

    RCC.enable_peripheral_clk(RCC_PERIPHERAL_UART0, true);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOA, true);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOB, true);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOC, true);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOD, true);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_PWR, true);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_RTC, true);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_SAC, true);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_LORA, true);

    // Turn the white LED on to know the board is alive. It will be turned off in app_start() when the device enters low power mode.
    GPIOA.init(GpioPin::COOL_WHITE_LED, GpioMode::OutputPPHigh);
    GPIOA.init(GpioPin::WARM_WHITE_LED, GpioMode::OutputPPLow);

    delay_ms(100);

    PWR.xo32k_lpm_cmd(true);

    uart_log_init();

    unsafe { ffi::RtcInit() };
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
