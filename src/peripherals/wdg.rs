use crate::{
    peripherals::{
        rcc::RCC_PERIPHERAL_WDG,
        regs::{RCC, RCC_RST_CR_WDG_RESET_REQ_EN_MASK, Wdg},
    },
    tremo_reg_en,
};

pub const WDG_LOCK_TOKEN: u32 = 0x1ACCE551;
pub const WDG_RESEN: u32 = 1 << 1;
pub const WDG_INTEN: u32 = 1;

impl Wdg {
    pub fn lock(&mut self) {
        self.lock = !WDG_LOCK_TOKEN;
    }

    pub fn unlock(&mut self) {
        self.lock = WDG_LOCK_TOKEN;
    }

    pub fn start(&mut self, reload_value: u32) {
        self.unlock();
        self.load = reload_value;
        self.control = WDG_RESEN | WDG_INTEN;
        self.lock();

        tremo_reg_en!(RCC.clone(), rst_cr, RCC_RST_CR_WDG_RESET_REQ_EN_MASK, true);
    }

    pub fn reload(&mut self) {
        self.unlock();
        self.intclr = 0x1;
        self.lock();
    }

    pub fn stop(&mut self) {
        tremo_reg_en!(RCC.clone(), rst_cr, RCC_RST_CR_WDG_RESET_REQ_EN_MASK, false);

        self.unlock();
        self.control = 0x0;
        self.load = 0xFFFFFFFF;
        self.lock();
    }
}

pub fn deinit() {
    let rcc = &mut RCC.clone();
    rcc.enable_peripheral_clk(RCC_PERIPHERAL_WDG, false);
    rcc.rst_peripheral(RCC_PERIPHERAL_WDG, true);
    rcc.rst_peripheral(RCC_PERIPHERAL_WDG, false);
}
