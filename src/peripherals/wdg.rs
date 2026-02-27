use crate::{
    peripherals::{
        rcc::RCC_PERIPHERAL_WDG,
        regs::{RCC, RCC_RST_CR_WDG_RESET_REQ_EN_MASK, Wdg},
    },
    toggle_reg_bits,
};

pub const WDG_LOCK_TOKEN: u32 = 0x1ACCE551;
pub const WDG_RESEN: u32 = 1 << 1;
pub const WDG_INTEN: u32 = 1;

impl Wdg {
    pub fn lock(&self) {
        self.lock.write(!WDG_LOCK_TOKEN);
    }

    pub fn unlock(&self) {
        self.lock.write(WDG_LOCK_TOKEN);
    }

    pub fn start(&self, reload_value: u32) {
        self.unlock();
        self.load.write(reload_value);
        self.control.write(WDG_RESEN | WDG_INTEN);
        self.lock();

        toggle_reg_bits!(RCC.rst_cr, RCC_RST_CR_WDG_RESET_REQ_EN_MASK, true);
    }

    pub fn reload(&self) {
        self.unlock();
        self.intclr.write(0x1);
        self.lock();
    }

    pub fn stop(&self) {
        toggle_reg_bits!(RCC.rst_cr, RCC_RST_CR_WDG_RESET_REQ_EN_MASK, false);

        self.unlock();
        self.control.write(0x0);
        self.load.write(0xFFFFFFFF);
        self.lock();
    }
}

pub fn deinit() {
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_WDG, false);
    RCC.rst_peripheral(RCC_PERIPHERAL_WDG, true);
    RCC.rst_peripheral(RCC_PERIPHERAL_WDG, false);
}
