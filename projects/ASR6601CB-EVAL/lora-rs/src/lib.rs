#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod class_c;
mod ffi;

use ffi::*;

use crate::class_c::app_start;

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    board_init();
    app_start();
}

fn uart_log_init() {
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

fn board_init() {
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

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { printf(c"PANICKED\r\n".as_ptr()) };
    loop {}
}
