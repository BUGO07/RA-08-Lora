use crate::{
    cortex::{VolatileRO, VolatileRW},
    define_reg,
    peripherals::{
        rcc::RCC_PERIPHERAL_WDG,
        regs::{RCC, RCC_RST_CR_WDG_RESET_REQ_EN_MASK},
    },
    toggle_reg_bits,
};

pub const WDG_LOCK_TOKEN: u32 = 0x1ACCE551;
pub const WDG_RESEN: u32 = 1 << 1;
pub const WDG_INTEN: u32 = 1;

define_reg! {
    Wdg
    __Wdg {
        load: VolatileRW<u32>,
        value: VolatileRO<u32>,
        control: VolatileRW<u32>,
        intclr: VolatileRW<u32>,
        ris: VolatileRO<u32>,
        mis: VolatileRO<u32>,
        dummy0: [VolatileRO<u32>; 0x2FA],
        lock: VolatileRW<u32>,
        dummy1: [VolatileRO<u32>; 0xBF],
        itcr: VolatileRW<u32>,
        itop: VolatileRW<u32>,
        dummy2: [VolatileRO<u32>; 0x32],
        periphid4: VolatileRO<u32>,
        periphid5: VolatileRO<u32>,
        periphid6: VolatileRO<u32>,
        periphid7: VolatileRO<u32>,
        periphid0: VolatileRO<u32>,
        periphid1: VolatileRO<u32>,
        periphid2: VolatileRO<u32>,
        periphid3: VolatileRO<u32>,
        pcellid0: VolatileRO<u32>,
        pcellid1: VolatileRO<u32>,
        pcellid2: VolatileRO<u32>,
        pcellid3: VolatileRO<u32>,
    }
}

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
