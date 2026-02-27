use crate::{
    cortex::{IRQType, LIBRARY_NORMAL_INTERRUPT_PRIORITY, NVIC_PRIO_BITS, SCB, nvic_set_priority},
    peripherals::{
        delay::delay_init,
        regs::{EFC, RCC, RCC_CGR0_AFEC_CLK_EN_MASK},
    },
    toggle_reg_bits,
};

// ! TODO: pretty this up

fn nvic_init() {
    nvic_set_priority(IRQType::PendSV, (1 << NVIC_PRIO_BITS) - 1);

    for i in 0..=IRQType::Iwdg as i32 {
        nvic_set_priority(IRQType::from_i32(i), LIBRARY_NORMAL_INTERRUPT_PRIORITY);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn system_init() {
    // FPU enable
    toggle_reg_bits!(SCB.cpacr, ((3 << (10 * 2)) | (3 << (11 * 2))), true);

    // enable afec clk
    toggle_reg_bits!(RCC.cgr0, RCC_CGR0_AFEC_CLK_EN_MASK, true);

    // set flash read number to 1
    EFC.timing_cfg
        .write(EFC.timing_cfg.read() & (!0xF0000) | (1 << 16));
    while EFC.sr.read() & 0x2 == 0 {}

    nvic_init();

    delay_init();
}
