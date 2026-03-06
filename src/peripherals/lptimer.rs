use crate::{
    cortex::{VolatileRO, VolatileRW},
    define_reg,
    peripherals::{
        rcc::{RCC_PERIPHERAL_LPTIMER0, RCC_PERIPHERAL_LPTIMER1},
        regs::{LPTIMER0, LPTIMER1, RCC},
    },
    toggle_reg_bits,
};

/// LPTIMER status flags (ISR register)
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerStatus {
    /// Compare match
    Cmpm = 0x1,
    /// Autoreload match
    Arrm = 0x2,
    /// External trigger edge event
    Exttrig = 0x4,
    /// Compare register update OK
    Cmpok = 0x8,
    /// Autoreload register update OK
    Arrok = 0x10,
    /// Counter direction change down to up
    Up = 0x20,
    /// Counter direction change up to down
    Down = 0x40,
    /// CFGR register operation status
    Cfgrok = 0x80,
    /// CR register operation status
    Crok = 0x100,
}

/// LPTIMER interrupt flags
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerInterrupt {
    /// Compare match interrupt
    Cmpm = 0x1,
    /// Autoreload match interrupt
    Arrm = 0x2,
    /// External trigger edge event interrupt
    Exttrig = 0x4,
    /// Compare register update OK interrupt
    Cmpok = 0x8,
    /// Autoreload register update OK interrupt
    Arrok = 0x10,
    /// Counter direction change down to up interrupt
    Up = 0x20,
    /// Counter direction change up to down interrupt
    Down = 0x40,
}

/// LPTIMER clear status success flags (CSR register)
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerClearStatusFlag {
    /// Compare match
    Cmpm = 0x1,
    /// Autoreload match
    Arrm = 0x2,
    /// External trigger edge event
    Exttrig = 0x4,
    /// Counter direction change down to up
    Up = 0x8,
    /// Counter direction change up to down
    Down = 0x10,
}

/// LPTIMER external trigger polarity
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerTrigPolarity {
    /// Software trigger
    Software = 0x0,
    /// Rising edge
    Rising = 0x20000,
    /// Falling edge
    Falling = 0x40000,
    /// Rising and falling edge
    RisingFalling = 0x60000,
}

/// LPTIMER trigger source selection
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerTrigSel {
    /// External trigger etr
    Sel0 = 0x0,
    /// External trigger comp0
    Sel1 = 0x2000,
    /// External trigger comp1
    Sel2 = 0x4000,
    /// External trigger rtc cyc
    Sel3 = 0x6000,
    /// External trigger rtc alarm0
    Sel4 = 0x8000,
    /// External trigger rtc alarm1
    Sel5 = 0xa000,
    /// External trigger gpio
    Sel6 = 0xc000,
    /// External trigger gpio
    Sel7 = 0xe000,
}

/// LPTIMER clock prescaler
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerPrescaler {
    /// 1 prescaler
    Div1 = 0x0,
    /// 2 prescaler
    Div2 = 0x200,
    /// 4 prescaler
    Div4 = 0x400,
    /// 8 prescaler
    Div8 = 0x600,
    /// 16 prescaler
    Div16 = 0x800,
    /// 32 prescaler
    Div32 = 0xa00,
    /// 64 prescaler
    Div64 = 0xc00,
    /// 128 prescaler
    Div128 = 0xe00,
}

/// LPTIMER trigger filter configuration
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerTrigFilter {
    /// No filter
    None = 0x0,
    /// Filter length is 2
    Len2 = 0x40,
    /// Filter length is 4
    Len4 = 0x80,
    /// Filter length is 8
    Len8 = 0xc0,
}

/// LPTIMER external clock filter configuration
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerClkFilter {
    /// No filter
    None = 0x0,
    /// Filter length is 2
    Len2 = 0x8,
    /// Filter length is 4
    Len4 = 0x10,
    /// Filter length is 8
    Len8 = 0x18,
}

/// LPTIMER clock polarity
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerClkPolarity {
    /// Count by rising edge
    Rising = 0x0,
    /// Count by falling edge
    Falling = 0x2,
    /// Count by both rising and falling edge
    Both = 0x4,
    /// Reserved
    Reserved = 0x6,
}

/// LPTIMER CFGR register bit definitions
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerCfgr {
    /// External clock filter enable
    CkfltEn = 0x20,
    /// Trigger filter enable
    TrgfltEn = 0x100,
    /// Timeout mode enable
    Timeout = 0x80000,
    /// Waveform shape
    Wave = 0x100000,
    /// Waveform shape polarity
    Wavpol = 0x200000,
    /// ARR and CMP register update mode (preload)
    Preload = 0x400000,
    /// Counter mode selection
    Countmode = 0x800000,
    /// Encoder mode enable
    Enc = 0x1000000,
}

/// LPTIMER wakeup configuration
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerWakeup {
    /// CMPM wakeup enable
    Cmpm = 0x2000000,
    /// ARRM wakeup enable
    Arrm = 0x4000000,
    /// EXTTRIG wakeup enable
    Exttrig = 0x8000000,
    /// UP wakeup enable
    Up = 0x10000000,
    /// DOWN wakeup enable
    Down = 0x20000000,
    /// OUT wakeup enable
    Out = 0x40000000,
}

/// LPTIMER count mode
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LptimerMode {
    /// Start in single mode
    Single = 0x2,
    /// Start in continuous mode
    Continuous = 0x4,
}

/// LPTIMER enable flag
const LPTIMER_CR_ENABLE: u32 = 0x1;

/// LPTIMER initialization configuration
pub struct LptimerConfig {
    /// Select external clock
    pub sel_external_clock: bool,
    /// Count by external clock
    pub count_by_external: bool,
    /// Clock prescaler
    pub prescaler: LptimerPrescaler,
    /// Auto-reload preload
    pub autoreload_preload: bool,
    /// Invert output wave polarity
    pub wavpol_inverted: bool,
}

define_reg! {
    Lptimer
    __Lptimer {
        /// LPTIMER flag and status register
        isr: VolatileRO<u32>,
        /// LPTIMER flag clear register
        icr: VolatileRW<u32>,
        /// LPTIMER interrupt enable register
        ier: VolatileRW<u32>,
        /// LPTIMER configuration register
        cfgr: VolatileRW<u32>,
        /// LPTIMER control register
        cr: VolatileRW<u32>,
        /// LPTIMER compare register
        cmp: VolatileRW<u32>,
        /// LPTIMER autoreload register
        arr: VolatileRW<u32>,
        /// LPTIMER counter register
        cnt: VolatileRO<u32>,
        /// LPTIMER CSR register
        csr: VolatileRO<u32>,
        /// LPTIMER SR1 register
        sr1: VolatileRO<u32>,
    }
}

impl Lptimer {
    /// Initialize the LPTIMER peripheral with the given configuration
    pub fn init(&self, config: LptimerConfig) {
        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(
            self.cfgr,
            LptimerCfgr::Countmode as u32,
            config.count_by_external
        );

        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(self.cfgr, LptimerPrescaler::Div128 as u32, false);

        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(self.cfgr, config.prescaler as u32, true);

        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(
            self.cfgr,
            LptimerCfgr::Preload as u32,
            config.autoreload_preload
        );

        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(
            self.cfgr,
            LptimerCfgr::Wavpol as u32,
            config.wavpol_inverted
        );

        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Deinitialize the LPTIMER peripheral
    pub fn deinit(&self) {
        if self.ptr() == LPTIMER0.ptr() {
            RCC.enable_peripheral_clk(RCC_PERIPHERAL_LPTIMER0, false);
            RCC.rst_peripheral(RCC_PERIPHERAL_LPTIMER0, true);
            RCC.rst_peripheral(RCC_PERIPHERAL_LPTIMER0, false);
        } else if self.ptr() == LPTIMER1.ptr() {
            RCC.enable_peripheral_clk(RCC_PERIPHERAL_LPTIMER1, false);
            RCC.rst_peripheral(RCC_PERIPHERAL_LPTIMER1, true);
            RCC.rst_peripheral(RCC_PERIPHERAL_LPTIMER1, false);
        }
    }

    /// Enable or disable the LPTIMER
    pub fn cmd(&self, state: bool) {
        toggle_reg_bits!(self.cr, LPTIMER_CR_ENABLE, state);
        while !self.get_status(LptimerStatus::Crok) {}
    }

    /// Enable or disable external clock filter
    pub fn enable_clock_filter(&self, enable: bool) {
        toggle_reg_bits!(self.cfgr, LptimerCfgr::CkfltEn as u32, enable);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Enable or disable trigger filter
    pub fn enable_trigger_filter(&self, enable: bool) {
        toggle_reg_bits!(self.cfgr, LptimerCfgr::TrgfltEn as u32, enable);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Configure wakeup source
    pub fn config_wakeup(&self, wkup: LptimerWakeup, enable: bool) {
        toggle_reg_bits!(self.cfgr, wkup as u32, enable);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Enable or disable timeout mode
    pub fn config_timeout(&self, enable: bool) {
        toggle_reg_bits!(self.cfgr, LptimerCfgr::Timeout as u32, enable);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Enable or disable waveform output
    pub fn config_wave(&self, enable: bool) {
        toggle_reg_bits!(self.cfgr, LptimerCfgr::Wave as u32, enable);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Enable or disable encoder mode
    pub fn config_encoder(&self, enable: bool) {
        toggle_reg_bits!(self.cfgr, LptimerCfgr::Enc as u32, enable);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Configure external trigger polarity
    pub fn config_trigger_polarity(&self, polarity: LptimerTrigPolarity) {
        toggle_reg_bits!(self.cfgr, LptimerTrigPolarity::RisingFalling as u32, false);
        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(self.cfgr, polarity as u32, true);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Configure trigger source
    pub fn config_trigger_source(&self, source: LptimerTrigSel) {
        toggle_reg_bits!(self.cfgr, LptimerTrigSel::Sel7 as u32, false);
        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(self.cfgr, source as u32, true);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Configure clock prescaler
    pub fn config_clock_prescaler(&self, prescaler: LptimerPrescaler) {
        toggle_reg_bits!(self.cfgr, LptimerPrescaler::Div128 as u32, false);
        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(self.cfgr, prescaler as u32, true);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Configure trigger filter
    pub fn config_trigger_filter(&self, filter: LptimerTrigFilter) {
        toggle_reg_bits!(self.cfgr, LptimerTrigFilter::Len8 as u32, false);
        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(self.cfgr, filter as u32, true);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Configure external clock filter
    pub fn config_clock_filter(&self, filter: LptimerClkFilter) {
        toggle_reg_bits!(self.cfgr, LptimerClkFilter::Len8 as u32, false);
        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(self.cfgr, filter as u32, true);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Configure clock polarity
    pub fn config_clock_polarity(&self, polarity: LptimerClkPolarity) {
        toggle_reg_bits!(self.cfgr, LptimerClkPolarity::Reserved as u32, false);
        while !self.get_status(LptimerStatus::Cfgrok) {}
        toggle_reg_bits!(self.cfgr, polarity as u32, true);
        while !self.get_status(LptimerStatus::Cfgrok) {}
    }

    /// Configure count mode (single or continuous)
    pub fn config_count_mode(&self, mode: LptimerMode, enable: bool) {
        toggle_reg_bits!(self.cr, mode as u32, enable);
        while !self.get_status(LptimerStatus::Crok) {}
    }

    /// Enable or disable an interrupt source
    pub fn config_interrupt(&self, interrupt: LptimerInterrupt, enable: bool) {
        toggle_reg_bits!(self.ier, interrupt as u32, enable);
    }

    /// Clear an interrupt flag
    pub fn clear_interrupt(&self, interrupt: LptimerInterrupt) {
        toggle_reg_bits!(self.icr, interrupt as u32, true);
    }

    /// Get the interrupt status from the SR1 register
    pub fn get_interrupt_status(&self, interrupt: LptimerInterrupt) -> bool {
        self.sr1.read() & (interrupt as u32) == (interrupt as u32)
    }

    /// Set the autoreload register value
    pub fn set_arr_register(&self, arr_value: u16) {
        self.arr.write(arr_value as u32);
        while !self.get_status(LptimerStatus::Arrok) {}
    }

    /// Set the compare register value
    pub fn set_cmp_register(&self, cmp_value: u16) {
        self.cmp.write(cmp_value as u32);
        while !self.get_status(LptimerStatus::Cmpok) {}
    }

    /// Get the ISR status for a given flag
    pub fn get_status(&self, status: LptimerStatus) -> bool {
        self.isr.read() & (status as u32) == (status as u32)
    }

    /// Get the clear status success flag from the CSR register
    pub fn get_clear_status_flag(&self, flag: LptimerClearStatusFlag) -> bool {
        self.csr.read() & (flag as u32) == (flag as u32)
    }
}
