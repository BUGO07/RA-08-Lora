#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![allow(non_snake_case)]

mod ffi;
use ffi::*;

const PERIPH_BASE: u32 = 0x40000000;
static mut UART0: *mut uart_t = (PERIPH_BASE + 0x3000) as *mut uart_t;
static mut GPIO_A: *mut gpio_t = GPIO_BASE as *mut gpio_t;
static mut GPIO_B: *mut gpio_t = (GPIO_BASE + 0x400) as *mut gpio_t;

#[unsafe(no_mangle)]
pub extern "C" fn uart_log_init() {
    unsafe {
        gpio_set_iomux(GPIO_B, GPIO_PIN_0, 1);
        gpio_set_iomux(GPIO_B, GPIO_PIN_1, 1);

        let mut uart_config: uart_config_t = core::mem::zeroed();
        uart_config_init(&mut uart_config);

        uart_config.baudrate = UART_BAUDRATE_115200;
        uart_init(UART0, &mut uart_config);
        uart_cmd(UART0, true);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn board_init() {
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

        gpio_init(GPIO_A, GPIO_PIN_3, GPIO_MODE_OUTPUT_PP_LOW);
        gpio_init(GPIO_A, GPIO_PIN_4, GPIO_MODE_OUTPUT_PP_LOW);
        gpio_init(GPIO_A, GPIO_PIN_5, GPIO_MODE_OUTPUT_PP_LOW);

        delay_ms(100);
        pwr_xo32k_lpm_cmd(true);

        uart_log_init();

        RtcInit();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    board_init();

    app_start();
}

use core::ptr;

const RF_FREQUENCY: u32 = 470000000;
const TX_OUTPUT_POWER: i8 = 14;

const RX_TIMEOUT_VALUE: u32 = 1800;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum States_t {
    LowPower,
    Rx,
    RxTimeout,
    RxError,
    Tx,
    TxTimeout,
}

static mut STATE: States_t = States_t::LowPower;

static mut BUFFER_SIZE: u16 = 5;
static mut BUFFER: [u8; 5] = [0; 5];

static mut RSSI_VALUE: i8 = 0;
static mut SNR_VALUE: i8 = 0;

static mut CHIP_ID: [u32; 2] = [0; 2];

static mut RADIO_EVENTS: RadioEvents_t = RadioEvents_t {
    TxDone: None,
    RxDone: None,
    TxTimeout: None,
    RxTimeout: None,
    RxError: None,
    FhssChangeChannel: None,
    CadDone: None,
};

extern "C" fn OnTxDone() {
    unsafe {
        (Radio.Sleep.unwrap())();
        STATE = States_t::Tx;
    }
}

extern "C" fn OnRxDone(payload: *mut u8, size: u16, rssi: i16, snr: i8) {
    unsafe {
        (Radio.Sleep.unwrap())();
        BUFFER_SIZE = size;

        ptr::copy_nonoverlapping(payload, BUFFER.as_mut_ptr(), BUFFER_SIZE as usize);

        RSSI_VALUE = rssi as i8;
        SNR_VALUE = snr;
        STATE = States_t::Rx;
    }
}

extern "C" fn OnTxTimeout() {
    unsafe {
        (Radio.Sleep.unwrap())();
        STATE = States_t::TxTimeout;
    }
}

extern "C" fn OnRxTimeout() {
    unsafe {
        printf(c"OnRxTimeout\r\n".as_ptr());
        (Radio.Sleep.unwrap())();
        STATE = States_t::RxTimeout;
    }
}

extern "C" fn OnRxError() {
    unsafe {
        (Radio.Sleep.unwrap())();
        STATE = States_t::RxError;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn app_start() -> ! {
    unsafe {
        let mut isMaster: bool = true;
        let mut i: u8;
        let mut random: u32;

        printf(c"PingPong test Start!\r\n".as_ptr());

        system_get_chip_id(CHIP_ID.as_mut_ptr());
        printf(c"CHIP_ID IS %d %d!\r\n".as_ptr(), CHIP_ID[0], CHIP_ID[1]);

        RADIO_EVENTS.TxDone = Some(OnTxDone);
        RADIO_EVENTS.RxDone = Some(OnRxDone);
        RADIO_EVENTS.TxTimeout = Some(OnTxTimeout);
        RADIO_EVENTS.RxTimeout = Some(OnRxTimeout);
        RADIO_EVENTS.RxError = Some(OnRxError);

        (Radio.Init.unwrap())(&mut RADIO_EVENTS);
        (Radio.SetChannel.unwrap())(RF_FREQUENCY);

        (Radio.SetTxConfig.unwrap())(
            MODEM_LORA,
            TX_OUTPUT_POWER,
            0,
            0,
            7,
            1,
            8,
            false,
            true,
            false,
            0,
            false,
            3000,
        );

        (Radio.SetRxConfig.unwrap())(
            MODEM_LORA, 0, 7, 1, 0, 8, 0, false, 0, true, false, 0, false, true,
        );

        (Radio.Rx.unwrap())(RX_TIMEOUT_VALUE);

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
                                gpio_toggle(GPIO_A, GPIO_PIN_5);
                            }
                            b'G' => {
                                gpio_toggle(GPIO_A, GPIO_PIN_4);
                            }
                            b'B' => {
                                gpio_toggle(GPIO_A, GPIO_PIN_3);
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
