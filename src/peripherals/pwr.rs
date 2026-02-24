// ! TODO: Implement Core Libraries to implement this

use crate::{
    analog_read, analog_write,
    ffi::PWR_LP_MODE_STOP3,
    peripherals::regs::{EFC, EFC_CR_PREFETCH_EN_MASK, Pwr},
    set_reg_bits, toggle_reg_bits,
};

///The bits mask of the low power mode register
pub const PWR_LP_MODE_MASK: u32 = 0x00000003;
/// The bit mask of the low power mode ext register
pub const PWR_LP_MODE_EXT_MASK: u32 = 1 << 24;

pub const REG_AFEC_RAW_SR: u32 = 0x40008208;
pub const AFEC_RAW_SR_RCO48M_READY: u32 = 0x00000004;

impl Pwr {
    #![allow(unused)]
    pub fn deep_sleep(&mut self, mode: u32, wfi: u32) {
        if (unsafe { *(0x10002010 as *const u32) } & 0x3) == 0 {
            set_reg_bits!(self, cr1, (0xF << 20), (1 << 20));
        }

        todo!("core_cm4");
        // toggle_reg_bits!(SCB, scr, SCB_SCR_SLEEPDEEP_Msk, true);

        toggle_reg_bits!(self, cr0, 1 << 5, wfi == 0);

        if mode < PWR_LP_MODE_STOP3 as u32 {
            set_reg_bits!(self, cr0, PWR_LP_MODE_MASK, mode);
        } else {
            if EFC.cr & EFC_CR_PREFETCH_EN_MASK != 0 {
                // flash_cr_unlock!();
                toggle_reg_bits!(EFC.clone(), cr, EFC_CR_PREFETCH_EN_MASK, false);
                // flash_cr_unlock!();
            }

            let value = analog_read!(0x0C);
            if value & (1 << 14) == 0 {
                analog_write!(0x0C, value | (1 << 14));
            }

            set_reg_bits!(self, cr0, PWR_LP_MODE_MASK, PWR_LP_MODE_STOP3);
            toggle_reg_bits!(
                self,
                cr1,
                PWR_LP_MODE_EXT_MASK,
                mode == PWR_LP_MODE_STOP3 as u32
            );
        }

        // if wfi != 0 {
        //     NVIC_EnableIRQ(PWR_IRQn);
        //     __WFI();
        // } else {
        //     __SEV();
        //     __WFE();
        //     __WFE();
        // }
    }

    pub fn deepsleep_wfi(&mut self, mode: u32) {
        self.deep_sleep(mode, 1);
    }

    pub fn deepsleep_wfe(&mut self, mode: u32) {
        self.deep_sleep(mode, 0);
    }

    pub fn sleep_wfi(&mut self, low_power: bool) {
        if low_power {
            self.enter_lprun_mode();
        }

        // NVIC_EnableIRQ(PWR_IRQn);
        // __WFI();
    }

    pub fn sleep_wfe(&mut self, low_power: bool) {
        if low_power {
            self.enter_lprun_mode();
        }

        // __WFE();
    }

    pub fn enter_lprun_mode(&mut self) {
        analog_write!(0x05, analog_read!(0x05) | (1 << 3));
        analog_write!(0x06, analog_read!(0x06) | (0x3 << 20));
    }

    pub fn exit_lprun_mode(&mut self) {
        analog_write!(0x06, analog_read!(0x06) & !(0x3 << 20));
        analog_write!(0x05, analog_read!(0x05) & !(1 << 3));
    }

    pub fn xo32k_lpm_cmd(&mut self, enable: bool) {
        let value = analog_read!(0x03);
        analog_write!(
            0x03,
            if enable {
                value | (1 << 7)
            } else {
                value & !(1 << 7)
            }
        );
    }
}
