#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

/// Class C LoRaWAN module
pub mod class_c;
/// C FFI Bindings for ASR6601 SDK
pub mod ffi;
/// Interrupts
pub mod tremo_it;

use ffi::*;

use crate::class_c::app_start;

/// entry point
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    board_init();
    app_start();
}

/// initialize UART for logging
pub fn uart_log_init() {
    unsafe {
        gpio_set_iomux(GPIOB, GPIO_PIN_0, 1);
        gpio_set_iomux(GPIOB, GPIO_PIN_1, 1);

        let mut uart_config: uart_config_t = core::mem::zeroed();
        uart_config_init(&mut uart_config);

        uart_config.baudrate = UART_BAUDRATE_115200;
        uart_init(UART0, &mut uart_config);
        uart_cmd(UART0, true);
    }
}

/// init board, enable peripheral clocks, etc.
pub fn board_init() {
    unsafe {
        rcc_enable_oscillator(RCC_OSC_XO32K, true);

        rcc_enable_peripheral_clk(RCC_PERIPHERAL_UART0, true);
        rcc_enable_peripheral_clk(RCC_PERIPHERAL_GPIOA, true);
        rcc_enable_peripheral_clk(RCC_PERIPHERAL_GPIOB, true);
        rcc_enable_peripheral_clk(RCC_PERIPHERAL_GPIOC, true);
        rcc_enable_peripheral_clk(RCC_PERIPHERAL_GPIOD, true);
        rcc_enable_peripheral_clk(RCC_PERIPHERAL_PWR, true);
        rcc_enable_peripheral_clk(RCC_PERIPHERAL_RTC, true);
        rcc_enable_peripheral_clk(RCC_PERIPHERAL_SAC, true);
        rcc_enable_peripheral_clk(RCC_PERIPHERAL_LORA, true);

        delay_ms(100);
        pwr_xo32k_lpm_cmd(true);

        uart_log_init();

        RtcInit();
    }
}

/// rust panic handler
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        if let Some(location) = info.location() {
            printf(
                c"Panicked at %s:%d:%d\r\n".as_ptr(),
                location.file_as_c_str().as_ptr(),
                location.line(),
                location.column(),
            );
        } else {
            printf(c"Panicked at unknown location\r\n".as_ptr());
        }
    };
    loop {}
}
