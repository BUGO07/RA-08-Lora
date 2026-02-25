use crate::{
    peripherals::{
        rcc::{
            RCC_PERIPHERAL_GPIOA, RCC_PERIPHERAL_GPIOB, RCC_PERIPHERAL_GPIOC, RCC_PERIPHERAL_GPIOD,
        },
        regs::{GPIOA_BASE, GPIOD_BASE, Gpio, RCC, SetStatus},
    },
    set_reg_bits, toggle_reg_bits,
};

/// GPIO pin
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GpioPin {
    /// GPIO Pin 0
    Pin0,
    /// GPIO Pin 1
    Pin1,
    /// GPIO Pin 2
    Pin2,
    /// GPIO Pin 3
    Pin3,
    /// GPIO Pin 4
    Pin4,
    /// GPIO Pin 5
    Pin5,
    /// GPIO Pin 6
    Pin6,
    /// GPIO Pin 7
    Pin7,
    /// GPIO Pin 8
    Pin8,
    /// GPIO Pin 9
    Pin9,
    /// GPIO Pin 10
    Pin10,
    /// GPIO Pin 11
    Pin11,
    /// GPIO Pin 12
    Pin12,
    /// GPIO Pin 13
    Pin13,
    /// GPIO Pin 14
    Pin14,
    /// GPIO Pin 15
    Pin15,
}

/// GPIO pin mode
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
    /// Push-Pull Output Low Level
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

/// GPIO interrupt type
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
    /// Init the GPIOx according to the specified parameters
    pub fn init(&self, gpio_pin: GpioPin, mode: GpioMode) {
        let pin = gpio_pin as u32;
        match mode {
            GpioMode::InputFloating => {
                toggle_reg_bits!(self, oer, 1 << pin, true);
                toggle_reg_bits!(self, ier, 1 << pin, true);
                toggle_reg_bits!(self, per, 1 << pin, false);
            }
            GpioMode::InputPullUp => {
                toggle_reg_bits!(self, oer, 1 << pin, true);
                toggle_reg_bits!(self, ier, 1 << pin, true);
                toggle_reg_bits!(self, per, 1 << pin, true);
                toggle_reg_bits!(self, psr, 1 << pin, true);
            }
            GpioMode::InputPullDown => {
                toggle_reg_bits!(self, oer, 1 << pin, true);
                toggle_reg_bits!(self, ier, 1 << pin, true);
                toggle_reg_bits!(self, per, 1 << pin, true);
                toggle_reg_bits!(self, psr, 1 << pin, false);
            }
            GpioMode::OutputPPHigh => {
                toggle_reg_bits!(self, oer, 1 << pin, false);
                toggle_reg_bits!(self, ier, 1 << pin, false);
                toggle_reg_bits!(self, otyper, 1 << pin, false);
                toggle_reg_bits!(self, odr, 1 << pin, true);
            }
            GpioMode::OutputPPLow => {
                toggle_reg_bits!(self, oer, 1 << pin, false);
                toggle_reg_bits!(self, ier, 1 << pin, false);
                toggle_reg_bits!(self, otyper, 1 << pin, false);
                toggle_reg_bits!(self, odr, 1 << pin, false);
            }
            GpioMode::OutputODHiz => {
                if self.ptr() as u32 == GPIOD_BASE && pin > GpioPin::Pin7 as u32 {
                    toggle_reg_bits!(self, odr, 1 << pin, false);
                    toggle_reg_bits!(self, ier, 1 << pin, false);
                    toggle_reg_bits!(self, oer, 1 << pin, true);
                    toggle_reg_bits!(self, psr, 1 << pin, true);
                } else {
                    toggle_reg_bits!(self, oer, 1 << pin, false);
                    toggle_reg_bits!(self, ier, 1 << pin, false);
                    toggle_reg_bits!(self, otyper, 1 << pin, true);
                    toggle_reg_bits!(self, odr, 1 << pin, true);
                }
            }
            GpioMode::OutputODLow => {
                if self.ptr() as u32 == GPIOD_BASE && pin > GpioPin::Pin7 as u32 {
                    toggle_reg_bits!(self, odr, 1 << pin, false);
                    toggle_reg_bits!(self, ier, 1 << pin, false);
                    toggle_reg_bits!(self, oer, 1 << pin, false);
                    toggle_reg_bits!(self, psr, 1 << pin, true);
                } else {
                    toggle_reg_bits!(self, oer, 1 << pin, false);
                    toggle_reg_bits!(self, ier, 1 << pin, false);
                    toggle_reg_bits!(self, otyper, 1 << pin, true);
                    toggle_reg_bits!(self, odr, 1 << pin, false);
                }
            }
            GpioMode::Analog => {
                toggle_reg_bits!(self, oer, 1 << pin, true);
                toggle_reg_bits!(self, ier, 1 << pin, false);
                toggle_reg_bits!(self, per, 1 << pin, false);
            }
        }
    }

    /// Set the output level of the GPIO pin (High = true, Low = false)
    pub fn write(&self, gpio_pin: GpioPin, high: bool) {
        let pin = gpio_pin as u32;
        if self.ptr() as u32 == GPIOD_BASE && pin > GpioPin::Pin7 as u32 {
            if (self.odr.read() & (1 << pin) == 0)
                && (self.ier.read() & (1 << pin) == 0)
                && (self.oer.read() & (1 << pin) != 0)
                && (self.psr.read() & (1 << pin) != 0)
            {
                if !high {
                    toggle_reg_bits!(self, odr, 1 << pin, false);
                    toggle_reg_bits!(self, ier, 1 << pin, false);
                    toggle_reg_bits!(self, oer, 1 << pin, false);
                    toggle_reg_bits!(self, psr, 1 << pin, true);
                }
            } else if self.odr.read() & (1 << pin) == 0
                && self.ier.read() & (1 << pin) == 0
                && self.oer.read() & (1 << pin) == 0
                && self.psr.read() & (1 << pin) != 0
            {
                toggle_reg_bits!(self, oer, 1 << pin, false);
                toggle_reg_bits!(self, ier, 1 << pin, false);
                toggle_reg_bits!(self, oer, 1 << pin, true);
                toggle_reg_bits!(self, psr, 1 << pin, true);
            } else {
                if high {
                    toggle_reg_bits!(self, bsr, 1 << pin, true);
                } else {
                    toggle_reg_bits!(self, brr, 1 << pin, true);
                }
            }
        } else {
            if high {
                toggle_reg_bits!(self, bsr, 1 << pin, true);
            } else {
                toggle_reg_bits!(self, brr, 1 << pin, true);
            }
        }
    }

    /// Read the input level (High = true, Low = false)
    pub fn read(&self, gpio_pin: GpioPin) -> bool {
        self.idr.read() & (1 << gpio_pin as u32) != 0
    }

    /// Toggle the output level of the GPIO pin
    pub fn toggle(&self, gpio_pin: GpioPin) {
        self.odr.write(self.odr.read() ^ (1 << gpio_pin as u32));
    }

    /// Config the ouput drive capability of the GPIO pin
    pub fn config_drive_capability(&self, gpio_pin: GpioPin, capability: GpioDriveCapability) {
        match capability {
            GpioDriveCapability::_4mA => {
                toggle_reg_bits!(self, dsr, 1 << gpio_pin as u32, true);
            }
            GpioDriveCapability::_8mA => {
                toggle_reg_bits!(self, dsr, 1 << gpio_pin as u32, false);
            }
        }
    }

    /// Config the interrupt type of the specified GPIO pin
    pub fn config_interrupt(&self, gpio_pin: GpioPin, int_type: IntType) {
        self.clear_interrupt(gpio_pin);
        set_reg_bits!(
            self,
            icr,
            0x3 << (2 * gpio_pin as u32),
            (int_type as u32) << (2 * gpio_pin as u32)
        );
    }

    /// Clear the interrupt of the specified GPIO pin
    pub fn clear_interrupt(&self, gpio_pin: GpioPin) {
        self.ifr
            .write(self.ifr.read() & 0x3 << (2 * gpio_pin as u32));
    }

    /// get the interrupt status of the specified GPIO pin
    pub fn get_interrupt_status(&self, gpio_pin: GpioPin) -> SetStatus {
        if self.ifr.read() & (0x3 << (2 * gpio_pin as u32)) != 0 {
            SetStatus::Set
        } else {
            SetStatus::Reset
        }
    }

    /// Config the wakeup setting of the specified GPIO pin
    pub fn config_wakeup(&self, gpio_pin: GpioPin, enable: bool, wake_up: bool) {
        toggle_reg_bits!(self, wucr, 1 << gpio_pin as u32, enable);
        toggle_reg_bits!(self, wulvl, 1 << gpio_pin as u32, wake_up);
    }

    /// Config the wakeup setting of the specified GPIO pin
    pub fn config_stop3_wakeup(&self, gpio_pin: GpioPin, enable: bool, wake_up: bool) {
        let mut pin = gpio_pin as u32;
        if self.ptr() as u32 == GPIOD_BASE && pin > GpioPin::Pin7 as u32 {
            return;
        }

        if self.ptr() as u32 == GPIOA_BASE {
            if matches!(gpio_pin, GpioPin::Pin6 | GpioPin::Pin7) {
                pin += 6;
            } else if matches!(gpio_pin, GpioPin::Pin12 | GpioPin::Pin13) {
                pin -= 6;
            }
        }

        let group = pin / 4;
        let offset = pin % 4;
        let tmp_mask = 0xF;
        let tmp = offset | if wake_up { 0x4 } else { 0x0 } | if enable { 0x8 } else { 0x0 };
        set_reg_bits!(
            self,
            stop3_wucr,
            tmp_mask << (4 * group),
            tmp << (4 * group)
        );
    }

    /// Config the iomux of the specified GPIO pin
    pub fn set_iomux(&self, gpio_pin: GpioPin, function: u8) {
        let pin = gpio_pin as u32;
        if pin > GpioPin::Pin7 as u32 {
            let index = pin - GpioPin::Pin8 as u32;
            let tmp_mask = if self.ptr() as u32 == GPIOD_BASE {
                0x7 << (3 * index)
            } else {
                0xF << (4 * index)
            };
            let tmp = if self.ptr() as u32 == GPIOD_BASE {
                function << (3 * index)
            } else {
                function << (4 * index)
            };
            set_reg_bits!(self, afrh, tmp_mask, tmp);
        } else {
            let tmp_mask = 0xF << (4 * pin);
            let tmp = function << (4 * pin);
            set_reg_bits!(self, afrl, tmp_mask, tmp);
        }
    }
}

/// Deinitializes the GPIO registers to the reset values
pub fn deinit() {
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOA, false);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOB, false);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOC, false);
    RCC.enable_peripheral_clk(RCC_PERIPHERAL_GPIOD, false);
    RCC.rst_peripheral(RCC_PERIPHERAL_GPIOA, true);
    RCC.rst_peripheral(RCC_PERIPHERAL_GPIOB, false);
}
