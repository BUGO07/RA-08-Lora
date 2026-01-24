#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod ffi;
mod radio;

use ffi::*;

use crate::radio::{BUFFER, BUFFER_SIZE, STATE, States_t};

const RX_TIMEOUT_VALUE: u32 = 1800;

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

        gpio_init(GPIOA, GPIO_PIN_4, GPIO_MODE_OUTPUT_PP_LOW);
        gpio_init(GPIOA, GPIO_PIN_5, GPIO_MODE_OUTPUT_PP_LOW);
        gpio_init(GPIOA, GPIO_PIN_14, GPIO_MODE_OUTPUT_PP_LOW);

        delay_ms(100);
        pwr_xo32k_lpm_cmd(true);

        uart_log_init();

        RtcInit();
    }
}

fn app_start() -> ! {
    unsafe {
        printf(c"RGB start!\r\n".as_ptr());

        radio::init();

        loop {
            if uart_get_flag_status(UART0, UART_FLAG_RX_FIFO_EMPTY as u32) == 0 {
                let recv = uart_receive_data(UART0);
                BUFFER[0] = recv;
                for i in 1..BUFFER_SIZE {
                    BUFFER[i as usize] = i as u8 - 1;
                }
                DelayMs(10);
                printf(c"Sent %c\r\n".as_ptr(), recv as u32);
                (Radio.Send.unwrap())(BUFFER.as_mut_ptr(), BUFFER_SIZE as u8);
            }
            match STATE {
                States_t::Rx => {
                    if BUFFER_SIZE > 0 {
                        printf(c"received %c\r\n".as_ptr(), BUFFER[0] as u32);
                        match BUFFER[0] {
                            b'R' => {
                                gpio_toggle(GPIOA, GPIO_PIN_5);
                            }
                            b'G' => {
                                gpio_toggle(GPIOA, GPIO_PIN_4);
                            }
                            b'B' => {
                                gpio_toggle(GPIOA, GPIO_PIN_14);
                            }
                            _ => {}
                        }
                        (Radio.Rx.unwrap())(RX_TIMEOUT_VALUE);
                    }
                    STATE = States_t::LowPower;
                }

                States_t::Tx => {
                    (Radio.Rx.unwrap())(RX_TIMEOUT_VALUE);
                    STATE = States_t::LowPower;
                }

                States_t::RxTimeout | States_t::RxError => {
                    (Radio.Rx.unwrap())(RX_TIMEOUT_VALUE);
                    STATE = States_t::LowPower;
                }

                States_t::TxTimeout => {
                    (Radio.Rx.unwrap())(RX_TIMEOUT_VALUE);
                    STATE = States_t::LowPower;
                }

                States_t::LowPower => {}
            }

            (Radio.IrqProcess.unwrap())();
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { printf(c"PANICKED\r\n".as_ptr()) };
    loop {}
}
