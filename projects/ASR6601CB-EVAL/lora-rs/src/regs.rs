use crate::{rcc::Rcc, uart::Uart};

const PERIPH_BASE: u32 = 0x40000000;
const UART0_BASE: u32 = PERIPH_BASE + 0x3000;
const UART1_BASE: u32 = PERIPH_BASE + 0x4000;
const UART2_BASE: u32 = PERIPH_BASE + 0x10000;
const UART3_BASE: u32 = PERIPH_BASE + 0x11000;
const RCC_BASE: u32 = PERIPH_BASE;

pub static mut UART0: Uart = Uart::new(UART0_BASE);
pub static mut UART1: Uart = Uart::new(UART1_BASE);
pub static mut UART2: Uart = Uart::new(UART2_BASE);
pub static mut UART3: Uart = Uart::new(UART3_BASE);
pub static mut RCC: Rcc = Rcc::new(RCC_BASE);

/// Set or clear bits in a register based on a mask and value
pub fn tremo_reg_set(reg: &mut u32, mask: u32, value: u32) {
    *reg = (*reg & !mask) | (value & mask);
}

/// Enable or disable bits in a register based on a mask
pub fn tremo_reg_en(reg: &mut u32, mask: u32, enable: bool) {
    if enable {
        *reg |= mask;
    } else {
        *reg &= !mask;
    }
}
