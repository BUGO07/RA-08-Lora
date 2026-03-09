#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

core::arch::global_asm!(include_str!("startup.S"));

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
        regs::{GPIOA, GPIOB, PWR, RCC, RTC, UART0},
    },
};

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

// use crate::lora::radio::{self, RadioEvents, RadioModem};
// use crate::lora::timer::{self, TimerEvent, TimerSysTime};

// fn dummy_cb() {
//     println!("Dummy callback fired!");
// }

// static mut TEST_TIMER_FIRED: bool = false;

// fn test_timer_cb() {
//     unsafe { TEST_TIMER_FIRED = true };
//     // Print via UART or toggle an LED
//     println!("Timer callback fired!");
// }

// pub fn run_smoke_tests() {
//     // --- Timer tests ---

//     // 1. TimerSysTime add/sub
//     let a = TimerSysTime {
//         seconds: 10,
//         subseconds: 500,
//     };
//     let b = TimerSysTime {
//         seconds: 5,
//         subseconds: 800,
//     };
//     let sum = timer::timer_add_sys_time(a, b);
//     assert_eq!(sum.seconds, 16);
//     assert_eq!(sum.subseconds, 300);

//     let c = TimerSysTime {
//         seconds: 10,
//         subseconds: 200,
//     };
//     let d = TimerSysTime {
//         seconds: 5,
//         subseconds: 800,
//     };
//     let diff = timer::timer_sub_sys_time(c, d);
//     assert_eq!(diff.seconds, 4);
//     assert_eq!(diff.subseconds, 400);

//     // 2. TimerInit
//     let mut ev = TimerEvent {
//         id: 0,
//         timestamp: 99,
//         reload_value: 99,
//         is_running: true,
//         callback: dummy_cb,
//     };
//     timer::timer_init(&mut ev, test_timer_cb);
//     assert_eq!(ev.timestamp, 0);
//     assert_eq!(ev.reload_value, 0);
//     assert!(!ev.is_running);

//     // 3. TimerSetValue
//     timer::timer_set_value(&mut ev, 1000);
//     assert_eq!(ev.reload_value, 1000);
//     assert_eq!(ev.timestamp, 1000);

//     // 4. TimerStart + wait + check callback fired
//     unsafe { TEST_TIMER_FIRED = false };
//     timer::timer_set_value(&mut ev, 100); // 100 ms
//     timer::timer_start(&mut ev);

//     // Busy-wait until the timer fires (or timeout after ~500ms)
//     let start = timer::timer_get_current_time();
//     while !unsafe { TEST_TIMER_FIRED } {
//         if timer::timer_get_elapsed_time(start) > 500 {
//             panic!("Timer callback never fired!");
//         }
//         // If your timer uses an IRQ, you may need to call
//         // timer::timer_irq_handler() here if it's polled
//     }

//     // 5. TimerStop (should not panic on a stopped timer)
//     timer::timer_stop(&mut ev);

//     // --- Radio tests ---

//     static EVENTS: RadioEvents = RadioEvents {
//         tx_done: None,
//         tx_timeout: None,
//         rx_done: None,
//         rx_timeout: None,
//         rx_error: None,
//         fhss_change_channel: None,
//         cad_done: None,
//     };

//     let ret = radio::radio_init(&EVENTS);
//     assert_eq!(ret, 0);

//     // Verify idle state after init
//     let state = radio::radio_get_status();
//     assert!(matches!(state, radio::RadioState::Idle));

//     // Set modem and channel
//     radio::radio_set_modem(RadioModem::LoRa);
//     radio::radio_set_channel(868_100_000);

//     // Configure RX
//     radio::radio_set_rx_config(
//         RadioModem::LoRa,
//         0, // BW 125kHz
//         7, // SF7
//         1, // CR 4/5
//         0,
//         8,
//         0,
//         false,
//         0,
//         true,
//         false,
//         0,
//         false,
//         true,
//     );

//     // Configure TX
//     radio::radio_set_tx_config(
//         RadioModem::LoRa,
//         14, // 14 dBm
//         0,
//         0,
//         7,
//         1,
//         8,
//         false,
//         true,
//         false,
//         0,
//         false,
//         3000,
//     );

//     // Time on air sanity check
//     let toa = radio::radio_time_on_air(RadioModem::LoRa, 10);
//     assert!(toa > 0, "Time on air should be positive");

//     // Random number
//     let rnd = radio::radio_random();
//     println!("Random number: {}", rnd);
//     let rnd = radio::radio_random();
//     println!("Random number: {}", rnd);
//     let rnd = radio::radio_random();
//     println!("Random number: {}", rnd);
//     // Can't assert much, just make sure it doesn't hang

//     // Put radio to sleep
//     radio::radio_sleep();

//     // If we get here, everything passed
//     // Print "ALL TESTS PASSED" via your UART
//     println!("ALL TESTS PASSED");
// }

/// entry point
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    board_init();
    // run_smoke_tests();
    // loop {}
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

    RTC.init();
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
