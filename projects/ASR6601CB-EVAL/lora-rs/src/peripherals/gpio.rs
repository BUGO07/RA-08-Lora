use crate::{
    ffi::{
        GPIO_PIN_6, GPIO_PIN_7, GPIO_PIN_8, GPIO_PIN_12, GPIO_PIN_13, RCC_PERIPHERAL_GPIOA,
        RCC_PERIPHERAL_GPIOB, RCC_PERIPHERAL_GPIOC, RCC_PERIPHERAL_GPIOD,
    },
    peripherals::regs::{GPIOA_BASE, GPIOD_BASE, Gpio, RCC, SetStatus},
    tremo_reg_en, tremo_reg_rd, tremo_reg_set, tremo_reg_wr,
};

#[repr(u32)]
pub enum GpioMode {
    /// Floating Input
    InputFloating,
    /// Pull-Up Input
    InputPullUp,
    /// Pull-Down Input
    InputPullDown,
    /// Push-Pull Output High Level
    OutputPPHigh,
    // Push-Pull Output Low Level
    OutputPPLow,
    /// Open-Drain Output High Impedance
    OutputODHiz,
    /// Open-Drain Output Low Level
    OutputODLow,
    /// Analog
    Analog,
}

/// The output drive capability of the GPIO pin
#[repr(u32)]
pub enum GpioDriveCapability {
    _4mA,
    _8mA,
}

#[repr(u32)]
pub enum IntType {
    /// Disable GPIO interrupt
    None,
    /// Interrupt on Rising Edge
    RisingEdge,
    /// Interrupt on Falling Edge
    FallingEdge,
    /// Interrupt on Rising and Falling Edge
    RisingFallingEdge,
}

impl Gpio {
    /// Deinitializes the GPIO registers to the reset values
    /// TODO: CURRENTLY IT DISABLES ALL GPIOS, FIXME
    pub fn deinit(&mut self) {
        unsafe {
            RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOA, false);
            RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOB, false);
            RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOC, false);
            RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOD, false);
            RCC.rst_peripheral(RCC_PERIPHERAL_GPIOA, true);
            RCC.rst_peripheral(RCC_PERIPHERAL_GPIOB, false);
        }
    }

    /// Init the GPIOx according to the specified parameters
    pub fn init(&mut self, pin: u8, mode: GpioMode) {
        // TODO: ASSERT GPIO MODE
        match mode {
            GpioMode::InputFloating => {
                tremo_reg_en!(self, oer, 1 << pin, true);
                tremo_reg_en!(self, ier, 1 << pin, true);
                tremo_reg_en!(self, per, 1 << pin, false);
            }
            GpioMode::InputPullUp => {
                tremo_reg_en!(self, oer, 1 << pin, true);
                tremo_reg_en!(self, ier, 1 << pin, true);
                tremo_reg_en!(self, per, 1 << pin, true);
                tremo_reg_en!(self, psr, 1 << pin, true);
            }
            GpioMode::InputPullDown => {
                tremo_reg_en!(self, oer, 1 << pin, true);
                tremo_reg_en!(self, ier, 1 << pin, true);
                tremo_reg_en!(self, per, 1 << pin, true);
                tremo_reg_en!(self, psr, 1 << pin, false);
            }
            GpioMode::OutputPPHigh => {
                tremo_reg_en!(self, oer, 1 << pin, false);
                tremo_reg_en!(self, ier, 1 << pin, false);
                tremo_reg_en!(self, otyper, 1 << pin, false);
                tremo_reg_en!(self, odr, 1 << pin, true);
            }
            GpioMode::OutputPPLow => {
                tremo_reg_en!(self, oer, 1 << pin, false);
                tremo_reg_en!(self, ier, 1 << pin, false);
                tremo_reg_en!(self, otyper, 1 << pin, false);
                tremo_reg_en!(self, odr, 1 << pin, false);
            }
            GpioMode::OutputODHiz => {
                if self.0 as u32 == GPIOD_BASE && pin > GPIO_PIN_7 {
                    tremo_reg_en!(self, odr, 1 << pin, false);
                    tremo_reg_en!(self, ier, 1 << pin, false);
                    tremo_reg_en!(self, oer, 1 << pin, true);
                    tremo_reg_en!(self, psr, 1 << pin, true);
                } else {
                    tremo_reg_en!(self, oer, 1 << pin, false);
                    tremo_reg_en!(self, ier, 1 << pin, false);
                    tremo_reg_en!(self, otyper, 1 << pin, true);
                    tremo_reg_en!(self, odr, 1 << pin, true);
                }
            }
            GpioMode::OutputODLow => {
                if self.0 as u32 == GPIOD_BASE && pin > GPIO_PIN_7 {
                    tremo_reg_en!(self, odr, 1 << pin, false);
                    tremo_reg_en!(self, ier, 1 << pin, false);
                    tremo_reg_en!(self, oer, 1 << pin, false);
                    tremo_reg_en!(self, psr, 1 << pin, true);
                } else {
                    tremo_reg_en!(self, oer, 1 << pin, false);
                    tremo_reg_en!(self, ier, 1 << pin, false);
                    tremo_reg_en!(self, otyper, 1 << pin, true);
                    tremo_reg_en!(self, odr, 1 << pin, false);
                }
            }
            GpioMode::Analog => {
                tremo_reg_en!(self, oer, 1 << pin, true);
                tremo_reg_en!(self, ier, 1 << pin, false);
                tremo_reg_en!(self, per, 1 << pin, false);
            }
        }
    }

    /// Set the output level of the GPIO pin (High = true, Low = false)
    pub fn write(&mut self, pin: u8, high: bool) {
        // TODO: ASSERT PIN
        if self.0 as u32 == GPIOD_BASE && pin > GPIO_PIN_7 {
            if (tremo_reg_rd!(self, odr) & (1 << pin) == 0)
                && (tremo_reg_rd!(self, ier) & (1 << pin) == 0)
                && (tremo_reg_rd!(self, oer) & (1 << pin) != 0)
                && (tremo_reg_rd!(self, psr) & (1 << pin) != 0)
            {
                if !high {
                    tremo_reg_en!(self, odr, 1 << pin, false);
                    tremo_reg_en!(self, ier, 1 << pin, false);
                    tremo_reg_en!(self, oer, 1 << pin, true);
                    tremo_reg_en!(self, psr, 1 << pin, true);
                }
            } else if tremo_reg_rd!(self, odr) & (1 << pin) == 0
                && tremo_reg_rd!(self, ier) & (1 << pin) == 0
                && tremo_reg_rd!(self, oer) & (1 << pin) == 0
                && tremo_reg_rd!(self, psr) & (1 << pin) != 0
            {
                tremo_reg_en!(self, oer, 1 << pin, false);
                tremo_reg_en!(self, ier, 1 << pin, false);
                tremo_reg_en!(self, oer, 1 << pin, true);
                tremo_reg_en!(self, psr, 1 << pin, true);
            } else {
                if high {
                    tremo_reg_en!(self, bsr, 1 << pin, true);
                } else {
                    tremo_reg_en!(self, brr, 1 << pin, true);
                }
            }
        }
    }

    /// Read the input level (High = true, Low = false)
    pub fn read(&self, pin: u8) -> bool {
        // TODO: ASSERT PIN
        tremo_reg_rd!(self, idr) & (1 << pin) != 0
    }

    /// Toggle the output level of the GPIO pin
    pub fn toggle(&mut self, pin: u8) {
        // TODO: ASSERT PIN
        tremo_reg_wr!(self, odr, tremo_reg_rd!(self, odr) ^ (1 << pin));
    }

    /// Config the ouput drive capability of the GPIO pin
    pub fn config_drive_capability(&mut self, pin: u8, capability: GpioDriveCapability) {
        // TODO: ASSERT PIN
        match capability {
            GpioDriveCapability::_4mA => {
                tremo_reg_en!(self, dsr, 1 << pin, true);
            }
            GpioDriveCapability::_8mA => {
                tremo_reg_en!(self, dsr, 1 << pin, false);
            }
        }
    }

    /// Config the interrupt type of the specified GPIO pin
    pub fn config_interrupt(&mut self, pin: u8, int_type: IntType) {
        // TODO: ASSERTS
        self.clear_interrupt(pin);
        tremo_reg_set!(self, icr, 0x3 << (2 * pin), (int_type as u32) << (2 * pin));
    }

    /// Clear the interrupt of the specified GPIO pin
    pub fn clear_interrupt(&mut self, pin: u8) {
        // TODO: ASSERT
        tremo_reg_wr!(self, ifr, tremo_reg_rd!(self, ifr) & 0x3 << (2 * pin));
    }

    /// get the interrupt status of the specified GPIO pin
    pub fn get_interrupt_status(&self, pin: u8) -> SetStatus {
        if tremo_reg_rd!(self, ifr) & (0x3 << (2 * pin)) != 0 {
            SetStatus::Set
        } else {
            SetStatus::Reset
        }
    }

    /// Config the wakeup setting of the specified GPIO pin
    pub fn config_wakeup(&mut self, pin: u8, enable: bool, wake_up: bool) {
        tremo_reg_en!(self, wucr, 1 << pin, enable);
        tremo_reg_en!(self, wulvl, 1 << pin, wake_up);
    }

    /// Config the wakeup setting of the specified GPIO pin
    pub fn config_stop3_wakeup(&mut self, mut pin: u8, enable: bool, wake_up: bool) {
        if self.0 as u32 == GPIOD_BASE && pin > GPIO_PIN_7 {
            return;
        }

        if self.0 as u32 == GPIOA_BASE {
            if matches!(pin, GPIO_PIN_6 | GPIO_PIN_7) {
                pin += 6;
            } else if matches!(pin, GPIO_PIN_12 | GPIO_PIN_13) {
                pin -= 6;
            }
        }

        let group = pin / 4;
        let offset = pin % 4;
        let tmp_mask = 0xF;
        let tmp = offset | if wake_up { 0x4 } else { 0x0 } | if enable { 0x8 } else { 0x0 };
        tremo_reg_set!(
            self,
            stop3_wucr,
            tmp_mask << (4 * group),
            tmp << (4 * group)
        );
    }

    /// Config the iomux of the specified GPIO pin
    pub fn set_iomux(&mut self, pin: u8, function: u8) {
        // TODO: ASSERT

        if pin > GPIO_PIN_7 {
            let index = pin - GPIO_PIN_8;
            let tmp_mask = if self.0 as u32 == GPIOD_BASE {
                0x7 << (3 * index)
            } else {
                0xF << (4 * index)
            };
            let tmp = if self.0 as u32 == GPIOD_BASE {
                function << (3 * index)
            } else {
                function << (4 * index)
            };
            tremo_reg_set!(self, afrh, tmp_mask, tmp);
        } else {
            let tmp_mask = 0xF << (4 * pin);
            let tmp = function << (4 * pin);
            tremo_reg_set!(self, afrl, tmp_mask, tmp);
        }
    }
}
