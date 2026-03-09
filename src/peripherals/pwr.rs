use crate::{
    analog_read, analog_write,
    cortex::{
        IRQType, SCB, SCB_SCR_SLEEPDEEP_MSK, VolatileRW,
        asm::{_sev, _wfe, _wfi},
        nvic_enable_irq,
    },
    define_reg,
    peripherals::{
        flash::{flash_cr_lock, flash_cr_unlock},
        regs::{EFC, EFC_CR_PREFETCH_EN_MASK},
    },
    set_reg_bits, toggle_reg_bits,
};

pub const PWR_LP_MODE_STOP0: usize = 0x00000000;
pub const PWR_LP_MODE_STOP1: usize = 0x00000001;
pub const PWR_LP_MODE_STOP2: usize = 0x00000002;
pub const PWR_LP_MODE_STOP3: usize = 0x00000003;
pub const PWR_LP_MODE_STANDBY: usize = 0x00000004;

///The bits mask of the low power mode register
pub const PWR_LP_MODE_MASK: usize = 0x00000003;
/// The bit mask of the low power mode ext register
pub const PWR_LP_MODE_EXT_MASK: usize = 1 << 24;

pub const REG_AFEC_RAW_SR: usize = 0x40008208;
pub const AFEC_RAW_SR_RCO48M_READY: usize = 0x00000004;

define_reg! {
    /// Wrapper over the raw PWR struct [`__Pwr`]
    Pwr
    /// Raw PWR struct
    __Pwr {
        /// control register 0, offset 0x00
        cr0: VolatileRW<usize>,
        /// control register 1, offset 0x04
        cr1: VolatileRW<usize>,
        /// status register 0, offset 0x08
        sr0: VolatileRW<usize>,
        /// status register 2, offset 0x0C
        sr1: VolatileRW<usize>,
        /// control register 3, offset 0x10
        cr2: VolatileRW<usize>,
        /// control register 4, offset 0x14
        cr3: VolatileRW<usize>,
        /// control register 5, offset 0x18
        cr4: VolatileRW<usize>,
        /// control register 6, offset 0x1C
        cr5: VolatileRW<usize>,
    }
}

impl Pwr {
    pub fn deep_sleep(&self, mode: usize, wfi: usize) {
        if (unsafe { *(0x10002010 as *const usize) } & 0x3) == 0 {
            set_reg_bits!(self.cr1, (0xF << 20), (1 << 20));
        }

        toggle_reg_bits!(SCB.scr, SCB_SCR_SLEEPDEEP_MSK, true);

        toggle_reg_bits!(self.cr0, 1 << 5, wfi == 0);

        if mode < PWR_LP_MODE_STOP3 {
            set_reg_bits!(self.cr0, PWR_LP_MODE_MASK, mode);
        } else {
            if EFC.cr.read() & EFC_CR_PREFETCH_EN_MASK != 0 {
                flash_cr_unlock();
                toggle_reg_bits!(EFC.cr, EFC_CR_PREFETCH_EN_MASK, false);
                flash_cr_lock();
            }

            let value = analog_read!(0x0C);
            if value & (1 << 14) == 0 {
                analog_write!(0x0C, value | (1 << 14));
            }

            set_reg_bits!(self.cr0, PWR_LP_MODE_MASK, PWR_LP_MODE_STOP3);
            toggle_reg_bits!(self.cr1, PWR_LP_MODE_EXT_MASK, mode == PWR_LP_MODE_STOP3);
        }

        if wfi != 0 {
            nvic_enable_irq(IRQType::Pwr);
            _wfi();
        } else {
            _sev();
            _wfe();
            _wfe();
        }
    }

    pub fn deepsleep_wfi(&self, mode: usize) {
        self.deep_sleep(mode, 1);
    }

    pub fn deepsleep_wfe(&self, mode: usize) {
        self.deep_sleep(mode, 0);
    }

    pub fn sleep_wfi(&self, low_power: bool) {
        if low_power {
            self.enter_lprun_mode();
        }

        nvic_enable_irq(IRQType::Pwr);
        _wfi();
    }

    pub fn sleep_wfe(&self, low_power: bool) {
        if low_power {
            self.enter_lprun_mode();
        }

        _wfe();
    }

    pub fn enter_lprun_mode(&self) {
        analog_write!(0x05, analog_read!(0x05) | (1 << 3));
        analog_write!(0x06, analog_read!(0x06) | (0x3 << 20));
    }

    pub fn exit_lprun_mode(&self) {
        analog_write!(0x06, analog_read!(0x06) & !(0x3 << 20));
        analog_write!(0x05, analog_read!(0x05) & !(1 << 3));
    }

    pub fn xo32k_lpm_cmd(&self, new_state: bool) {
        let value = analog_read!(0x03);
        analog_write!(
            0x03,
            if new_state {
                value | (1 << 7)
            } else {
                value & !(1 << 7)
            }
        );
    }
}
