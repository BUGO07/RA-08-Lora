use crate::{
    cortex::{VolatileRO, VolatileRW},
    define_reg,
    peripherals::{
        rcc::RCC_PERIPHERAL_WDG,
        regs::{RCC, RCC_RST_CR_WDG_RESET_REQ_EN_MASK},
    },
    toggle_reg_bits,
};

pub const WDG_LOCK_TOKEN: usize = 0x1ACCE551;
pub const WDG_RESEN: usize = 1 << 1;
pub const WDG_INTEN: usize = 1;

define_reg! {
    Wdg
    __Wdg {
        load: VolatileRW<usize>,
        value: VolatileRO<usize>,
        control: VolatileRW<usize>,
        intclr: VolatileRW<usize>,
        ris: VolatileRO<usize>,
        mis: VolatileRO<usize>,
        dummy0: [VolatileRO<usize>; 0x2FA],
        lock: VolatileRW<usize>,
        dummy1: [VolatileRO<usize>; 0xBF],
        itcr: VolatileRW<usize>,
        itop: VolatileRW<usize>,
        dummy2: [VolatileRO<usize>; 0x32],
        periphid4: VolatileRO<usize>,
        periphid5: VolatileRO<usize>,
        periphid6: VolatileRO<usize>,
        periphid7: VolatileRO<usize>,
        periphid0: VolatileRO<usize>,
        periphid1: VolatileRO<usize>,
        periphid2: VolatileRO<usize>,
        periphid3: VolatileRO<usize>,
        pcellid0: VolatileRO<usize>,
        pcellid1: VolatileRO<usize>,
        pcellid2: VolatileRO<usize>,
        pcellid3: VolatileRO<usize>,
    }
}

impl Wdg {
    pub fn lock(&self) {
        self.lock.write(!WDG_LOCK_TOKEN);
    }

    pub fn unlock(&self) {
        self.lock.write(WDG_LOCK_TOKEN);
    }

    pub fn start(&self, reload_value: usize) {
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
