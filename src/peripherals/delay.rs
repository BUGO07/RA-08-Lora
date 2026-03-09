use core::sync::atomic::{AtomicU8, AtomicU16, Ordering};

use crate::{
    cortex::{SYSTICK, systick_config},
    peripherals::{rcc::RCC_HCLK, regs::RCC},
};

static FAC_US: AtomicU8 = AtomicU8::new(0);
static FAC_MS: AtomicU16 = AtomicU16::new(0);

/// Initialize the registers used in delay functions
/// Must be called before using delay functions
pub fn delay_init() {
    let tick_rate = 1000;

    let clk_freq = RCC.get_clk_freq(RCC_HCLK);
    if clk_freq < 1000000 {
        panic!("Clock frequency is too low for delay functions");
    }

    FAC_US.store((clk_freq / 1000000) as u8, Ordering::Relaxed);
    let reload = clk_freq / tick_rate;

    FAC_MS.store((1000 / tick_rate) as u16, Ordering::Relaxed);

    systick_config(reload).expect("Could not configure SysTick");
}

/// Delay for a specified number of microseconds
/// Note: This function is blocking and will busy-wait, so it should be used for short delays only
#[unsafe(no_mangle)]
pub extern "C" fn delay_us(micros: usize) {
    if FAC_US.load(Ordering::Relaxed) == 0 {
        panic!("delay_init() must be called before using delay functions");
    }

    let mut tnow;
    let mut tcurrent = 0;

    let reload = SYSTICK.load.read();

    let ticks = micros * FAC_US.load(Ordering::Relaxed) as usize;
    let mut tprev = SYSTICK.val.read();

    loop {
        tnow = SYSTICK.val.read();
        if tnow != tprev {
            if tnow < tprev {
                tcurrent += tprev - tnow;
            } else {
                tcurrent += reload - tnow + tprev;
            }

            tprev = tnow;

            if tcurrent >= ticks {
                break;
            }
        }
    }
}

/// Delay for a specified number of milliseconds
/// Note: This function is blocking and will busy-wait, so it should be used for short delays only
#[unsafe(no_mangle)]
pub extern "C" fn delay_ms(millis: usize) {
    delay_us(millis * 1000);
}

#[unsafe(no_mangle)]
pub extern "C" fn DelayMs(millis: usize) {
    delay_ms(millis);
}
