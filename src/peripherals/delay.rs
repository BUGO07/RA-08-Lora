use core::sync::atomic::{AtomicU8, AtomicU16, Ordering};

use crate::{
    cortex::{SYSTICK, systick_config},
    peripherals::{rcc::RCC_HCLK, regs::RCC},
};

static FAC_US: AtomicU8 = AtomicU8::new(0);
static FAC_MS: AtomicU16 = AtomicU16::new(0);

pub fn delay_init() {
    let tick_rate = 1000;

    let clk_freq = RCC.get_clk_freq(RCC_HCLK);
    if clk_freq < 1000000 {
        return; // TODO: maybe return an error instead of silently doing nothing
    }

    FAC_US.store((clk_freq / 1000000) as u8, Ordering::Relaxed);
    let reload = clk_freq / tick_rate;

    FAC_MS.store((1000 / tick_rate) as u16, Ordering::Relaxed);

    systick_config(reload).unwrap();
}

#[unsafe(no_mangle)]
pub extern "C" fn delay_us(nus: u32) {
    if FAC_US.load(Ordering::Relaxed) == 0 {
        return; // TODO: maybe return an error instead of silently doing nothing
    }

    let mut tnow;
    let mut tcnt = 0;

    let reload = SYSTICK.load.read();

    let ticks = nus * FAC_US.load(Ordering::Relaxed) as u32;
    let mut tpre = SYSTICK.val.read();

    loop {
        tnow = SYSTICK.val.read();
        if tnow != tpre {
            if tnow < tpre {
                tcnt += tpre - tnow;
            } else {
                tcnt += reload - tnow + tpre;
            }

            tpre = tnow;

            if tcnt >= ticks {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn delay_ms(nms: u32) {
    delay_us(nms * 1000);
}
