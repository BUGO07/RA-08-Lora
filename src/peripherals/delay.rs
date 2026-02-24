#![allow(unused)]

// ! TODO: Implement SysTick

use crate::peripherals::{rcc::RCC_HCLK, regs::RCC};

static mut FAC_US: u8 = 0;
static mut FAC_MS: u16 = 0;

pub fn delay_init() {
    let tick_rate = 1000;

    let clk_freq = RCC.get_clk_freq(RCC_HCLK);
    if clk_freq < 1000000 {
        return; // TODO: maybe return an error instead of silently doing nothing
    }

    unsafe { FAC_US = (clk_freq / 1000000) as u8 };
    let reload = (clk_freq / tick_rate) as u16;

    unsafe { FAC_MS = (1000 / tick_rate) as u16 };

    todo!("SysTick");
    // SysTick_Config(reload);
}

pub fn delay_us(nus: u32) {
    if unsafe { FAC_US } == 0 {
        return; // TODO: maybe return an error instead of silently doing nothing
    }

    let ticks = nus * unsafe { FAC_US as u32 };
    let reload = 0; // SysTick.LOAD
    let mut tpre = 0; // SysTick.VAL
    let mut tnow = 0;
    let mut tcnt = 0;
    loop {
        tnow = 0; // SysTick.VAL
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

pub fn delay_ms(nms: u32) {
    delay_us(nms * 1000);
}
