use crate::{cortex::nvic_system_reset, peripherals::regs::EFC};

pub fn system_reset() {
    nvic_system_reset();
}

pub fn system_get_chip_id() -> [u32; 2] {
    [EFC.sn_l.read(), EFC.sn_h.read()]
}
