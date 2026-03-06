use crate::{
    cortex::{VolatileRO, VolatileRW},
    define_reg,
    peripherals::{
        rcc::RCC_PERIPHERAL_IWDG,
        regs::{
            IWDG_CR_PREDIV_MASK, IWDG_CR_RSTEN_MASK, IWDG_CR_START_MASK, IWDG_CR_WKEN_MASK,
            IWDG_CR1_RESET_REQ_INT_EN_MASK, IWDG_SR_WRITE_CR_DONE, IWDG_SR_WRITE_SR2_DONE,
            IWDG_SR2_RESET_REQ_SR_MASK, RCC, RCC_RST_CR_IWDG_RESET_REQ_EN_MASK,
        },
    },
    set_reg_bits, toggle_reg_bits,
};

/// The maximum value of the IWDG reload
pub const IWDG_MAX_RELOAD: u32 = 0x0FFF;

#[repr(u32)]
pub enum IwdgPrescaler {
    Div4 = 0x00000000,
    Div8 = 0x00000002,
    Div16 = 0x00000004,
    Div32 = 0x00000006,
    Div64 = 0x00000008,
    Div128 = 0x0000000A,
    Div256 = 0x0000000C,
}

define_reg! {
    Iwdg
    __Iwdg {
        cr: VolatileRW<u32>,
        max: VolatileRW<u32>,
        win: VolatileRW<u32>,
        sr: VolatileRO<u32>,
        sr1: VolatileRO<u32>,
        cr1: VolatileRW<u32>,
        sr2: VolatileRW<u32>,
    }
}

impl Iwdg {
    fn wait_until_done(&self) {
        while self.sr.read() & 0xF != 0xF {}
    }

    pub fn deinit(&self) {
        RCC.enable_peripheral_clk(RCC_PERIPHERAL_IWDG, false);
        RCC.rst_peripheral(RCC_PERIPHERAL_IWDG, true);
        RCC.rst_peripheral(RCC_PERIPHERAL_IWDG, false);
    }

    pub fn init(&self, auto_reset: bool) {
        self.wait_until_done();
        toggle_reg_bits!(self.cr, IWDG_CR_START_MASK, false);

        self.wait_until_done();
        self.sr2.write(IWDG_SR2_RESET_REQ_SR_MASK);
        self.wait_until_done();

        toggle_reg_bits!(RCC.rst_cr, RCC_RST_CR_IWDG_RESET_REQ_EN_MASK, auto_reset);
        toggle_reg_bits!(self.cr, IWDG_CR_RSTEN_MASK, auto_reset);
        self.wait_until_done();
    }

    pub fn set_prescaler(&self, prescaler: IwdgPrescaler) {
        self.wait_until_done();

        set_reg_bits!(self.cr, IWDG_CR_PREDIV_MASK, prescaler as u32);
    }

    pub fn set_reload(&self, value: u32) {
        self.wait_until_done();
        self.max.write(value.min(IWDG_MAX_RELOAD));
    }

    pub fn set_window_value(&self, value: u32) {
        self.wait_until_done();

        self.win.write(value.min(IWDG_MAX_RELOAD));
    }

    pub fn reload(&self) {
        if self.sr2.read() & IWDG_SR2_RESET_REQ_SR_MASK != 0 {
            self.wait_until_done();
            self.sr2.write(IWDG_SR2_RESET_REQ_SR_MASK);
        }

        self.wait_until_done();
        self.max.write(self.max.read()); // ? ig changing the register has some sort of event happen
    }

    pub fn start(&self) {
        self.wait_until_done();

        toggle_reg_bits!(self.cr, IWDG_CR_START_MASK | IWDG_CR_WKEN_MASK, true);

        while self.sr.read() & IWDG_SR_WRITE_CR_DONE == 0 {}
    }

    pub fn stop(&self) {
        self.wait_until_done();

        toggle_reg_bits!(self.cr, IWDG_CR_START_MASK, false);

        while self.sr.read() & IWDG_SR_WRITE_CR_DONE == 0 {}
    }

    pub fn config_interrupt(&self, enable: bool) {
        toggle_reg_bits!(self.cr1, IWDG_CR1_RESET_REQ_INT_EN_MASK, enable);
    }

    pub fn clear_interrupt(&self) {
        self.sr2.write(IWDG_SR2_RESET_REQ_SR_MASK);

        while self.sr.read() & IWDG_SR_WRITE_SR2_DONE == 0 {}
    }
}
