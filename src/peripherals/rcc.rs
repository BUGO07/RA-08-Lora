use crate::{analog_read, analog_write, peripherals::regs::*, set_reg_bits, toggle_reg_bits};

pub const RCC_FREQ_48M: u32 = 48000000;
pub const RCC_FREQ_40M: u32 = 40000000;
pub const RCC_FREQ_32M: u32 = 32000000;
pub const RCC_FREQ_30M: u32 = 30000000;
pub const RCC_FREQ_24M: u32 = 24000000;
pub const RCC_FREQ_4M: u32 = 3600000;
pub const RCC_FREQ_32768: u32 = 32768;
pub const RCC_FREQ_32000: u32 = 32000;

/// RCC CLK type
pub const RCC_SYS_CLK: u32 = 0;
pub const RCC_HCLK: u32 = 1;
pub const RCC_PCLK0: u32 = 2;
pub const RCC_PCLK1: u32 = 3;

/// RCC oscillator type
pub const RCC_OSC_RCO48M: u32 = 0;
pub const RCC_OSC_RCO32K: u32 = 1;
pub const RCC_OSC_XO32K: u32 = 2;
pub const RCC_OSC_XO24M: u32 = 3;
pub const RCC_OSC_XO32M: u32 = 4;
pub const RCC_OSC_RCO4M: u32 = 5;

/// The source of the SYSCLK
pub const RCC_SYS_CLK_SOURCE_RCO48M_DIV2: u32 = RCC_CR0_SYSCLK_SEL_RCO48M_DIV2;
pub const RCC_SYS_CLK_SOURCE_RCO48M: u32 = RCC_CR0_SYSCLK_SEL_RCO48M;
pub const RCC_SYS_CLK_SOURCE_RCO32K: u32 = RCC_CR0_SYSCLK_SEL_RCO32K;
pub const RCC_SYS_CLK_SOURCE_XO32K: u32 = RCC_CR0_SYSCLK_SEL_XO32K;
pub const RCC_SYS_CLK_SOURCE_XO24M: u32 = RCC_CR0_SYSCLK_SEL_XO24M;
pub const RCC_SYS_CLK_SOURCE_XO32M: u32 = RCC_CR0_SYSCLK_SEL_XO32M;
pub const RCC_SYS_CLK_SOURCE_RCO4M: u32 = RCC_CR0_SYSCLK_SEL_RCO4M;

/// The source of the SYSTICK
pub const RCC_SYSTICK_SOURCE_XO32K: u32 = RCC_CR0_STCLKEN_SEL_XO32K;
pub const RCC_SYSTICK_SOURCE_RCO32K: u32 = RCC_CR0_STCLKEN_SEL_RCO32K;
pub const RCC_SYSTICK_SOURCE_HCLK: u32 = RCC_CR0_STCLKEN_SEL_RCO32K + 1;

/// The source of the MCOCLK
pub const RCC_MCO_CLK_SOURCE_RCO32K: u32 = RCC_CR0_MCO_CLK_SEL_RCO32K;
pub const RCC_MCO_CLK_SOURCE_XO32K: u32 = RCC_CR0_MCO_CLK_SEL_XO32K;
pub const RCC_MCO_CLK_SOURCE_RCO4M: u32 = RCC_CR0_MCO_CLK_SEL_RCO4M;
pub const RCC_MCO_CLK_SOURCE_XO24M: u32 = RCC_CR0_MCO_CLK_SEL_XO24M;
pub const RCC_MCO_CLK_SOURCE_XO32M: u32 = RCC_CR0_MCO_CLK_SEL_XO32M;
pub const RCC_MCO_CLK_SOURCE_RCO48M: u32 = RCC_CR0_MCO_CLK_SEL_RCO48M;
pub const RCC_MCO_CLK_SOURCE_SYSCLK: u32 = RCC_CR0_MCO_CLK_SEL_SYSCLCK;

/// The source of the UART0 CLK
pub const RCC_UART0_CLK_SOURCE_PCLK0: u32 = RCC_CR2_UART0_CLK_SEL_PCLK0;
pub const RCC_UART0_CLK_SOURCE_RCO4M: u32 = RCC_CR2_UART0_CLK_SEL_RCO4M;
pub const RCC_UART0_CLK_SOURCE_XO32K: u32 = RCC_CR2_UART0_CLK_SEL_XO32K;
pub const RCC_UART0_CLK_SOURCE_XO24M: u32 = RCC_CR2_UART0_CLK_SEL_XO24M;

/// The source of the UART1 CLK
pub const RCC_UART1_CLK_SOURCE_PCLK0: u32 = RCC_CR2_UART1_CLK_SEL_PCLK0;
pub const RCC_UART1_CLK_SOURCE_RCO4M: u32 = RCC_CR2_UART1_CLK_SEL_RCO4M;
pub const RCC_UART1_CLK_SOURCE_XO32K: u32 = RCC_CR2_UART1_CLK_SEL_XO32K;
pub const RCC_UART1_CLK_SOURCE_XO24M: u32 = RCC_CR2_UART1_CLK_SEL_XO24M;

/// The source of the UART2 CLK
pub const RCC_UART2_CLK_SOURCE_PCLK1: u32 = RCC_CR2_UART2_CLK_SEL_PCLK1;
pub const RCC_UART2_CLK_SOURCE_RCO4M: u32 = RCC_CR2_UART2_CLK_SEL_RCO4M;
pub const RCC_UART2_CLK_SOURCE_XO32K: u32 = RCC_CR2_UART2_CLK_SEL_XO32K;
pub const RCC_UART2_CLK_SOURCE_XO24M: u32 = RCC_CR2_UART2_CLK_SEL_XO24M;

/// The source of the UART3 CLK
pub const RCC_UART3_CLK_SOURCE_PCLK1: u32 = RCC_CR2_UART3_CLK_SEL_PCLK1;
pub const RCC_UART3_CLK_SOURCE_RCO4M: u32 = RCC_CR2_UART3_CLK_SEL_RCO4M;
pub const RCC_UART3_CLK_SOURCE_XO32K: u32 = RCC_CR2_UART3_CLK_SEL_XO32K;
pub const RCC_UART3_CLK_SOURCE_XO24M: u32 = RCC_CR2_UART3_CLK_SEL_XO24M;

/// The source of the LPTIMER0 CLK
pub const RCC_LPTIMER0_CLK_SOURCE_PCLK0: u32 = RCC_CR1_LPTIMER0_CLK_SEL_PCLK0;
pub const RCC_LPTIMER0_CLK_SOURCE_RCO4M: u32 = RCC_CR1_LPTIMER0_CLK_SEL_RCO4M;
pub const RCC_LPTIMER0_CLK_SOURCE_XO32K: u32 = RCC_CR1_LPTIMER0_CLK_SEL_XO32K;
pub const RCC_LPTIMER0_CLK_SOURCE_RCO32K: u32 = RCC_CR1_LPTIMER0_CLK_SEL_RCO32K;
pub const RCC_LPTIMER0_CLK_SOURCE_EXTCLK: u32 = RCC_CR1_LPTIMER0_CLK_SEL_RCO32K + 1;

/// The source of the LPTIMER1 CLK
pub const RCC_LPTIMER1_CLK_SOURCE_PCLK0: u32 = RCC_CR1_LPTIMER1_CLK_SEL_PCLK0;
pub const RCC_LPTIMER1_CLK_SOURCE_RCO4M: u32 = RCC_CR1_LPTIMER1_CLK_SEL_RCO4M;
pub const RCC_LPTIMER1_CLK_SOURCE_XO32K: u32 = RCC_CR1_LPTIMER1_CLK_SEL_XO32K;
pub const RCC_LPTIMER1_CLK_SOURCE_RCO32K: u32 = RCC_CR1_LPTIMER1_CLK_SEL_RCO32K;
pub const RCC_LPTIMER1_CLK_SOURCE_EXTCLK: u32 = RCC_CR1_LPTIMER1_CLK_SEL_RCO32K + 1;

/// The source of the LCD CLK
pub const RCC_LCD_CLK_SOURCE_XO32K: u32 = RCC_CR1_LCD_CLK_SEL_XO32K;
pub const RCC_LCD_CLK_SOURCE_RCO32K: u32 = RCC_CR1_LCD_CLK_SEL_RCO32K;
pub const RCC_LCD_CLK_SOURCE_RCO4M: u32 = RCC_CR1_LCD_CLK_SEL_RCO4M;

/// The source of the LPUART CLK
pub const RCC_LPUART_CLK_SOURCE_XO32K: u32 = RCC_CR1_LPUART_CLK_SEL_XO32K;
pub const RCC_LPUART_CLK_SOURCE_RCO32K: u32 = RCC_CR1_LPUART_CLK_SEL_RCO32K;
pub const RCC_LPUART_CLK_SOURCE_RCO4M: u32 = RCC_CR1_LPUART_CLK_SEL_RCO4M;

/// The source of the RTC CLK
pub const RCC_RTC_CLK_SOURCE_XO32K: u32 = RCC_CR1_RTC_CLK_SEL_XO32K;
pub const RCC_RTC_CLK_SOURCE_RCO32K: u32 = RCC_CR1_RTC_CLK_SEL_RCO32K;

/// The source of the IWDG CLK
pub const RCC_IWDG_CLK_SOURCE_XO32K: u32 = RCC_CR1_IWDG_CLK_SEL_XO32K;
pub const RCC_IWDG_CLK_SOURCE_RCO32K: u32 = RCC_CR1_IWDG_CLK_SEL_RCO32K;

/// The source of the ADC CLK
pub const RCC_ADC_CLK_SOURCE_PCLK1: u32 = RCC_CR2_ADC_CLK_SEL_PCLK1;
pub const RCC_ADC_CLK_SOURCE_SYSCLK: u32 = RCC_CR2_ADC_CLK_SEL_SYSCLK;
pub const RCC_ADC_CLK_SOURCE_RCO48M: u32 = RCC_CR2_ADC_CLK_SEL_RCO48M;

/// The source of the QSPI CLK
pub const RCC_QSPI_CLK_SOURCE_HCLK: u32 = RCC_CR2_QSPI_CLK_SEL_HCLK;
pub const RCC_QSPI_CLK_SOURCE_SYSCLK: u32 = RCC_CR2_QSPI_CLK_SEL_SYSCLK;

/// The source of the I2S CLK
pub const RCC_I2S_CLK_SOURCE_PCLK0: u32 = RCC_CR2_I2S_CLK_SEL_PCLK0;
pub const RCC_I2S_CLK_SOURCE_XO24M: u32 = RCC_CR2_I2S_CLK_SEL_XO24M;
pub const RCC_I2S_CLK_SOURCE_XO32M: u32 = RCC_CR2_I2S_CLK_SEL_XO32M;
pub const RCC_I2S_CLK_SOURCE_EXT_CLK: u32 = RCC_CR2_I2S_CLK_SEL_EXT_CLK;

/// The divider of the HCLK
pub const RCC_HCLK_DIV_1: u32 = RCC_CR0_HCLK_DIV_1;
pub const RCC_HCLK_DIV_2: u32 = RCC_CR0_HCLK_DIV_2;
pub const RCC_HCLK_DIV_4: u32 = RCC_CR0_HCLK_DIV_4;
pub const RCC_HCLK_DIV_8: u32 = RCC_CR0_HCLK_DIV_8;
pub const RCC_HCLK_DIV_16: u32 = RCC_CR0_HCLK_DIV_16;
pub const RCC_HCLK_DIV_32: u32 = RCC_CR0_HCLK_DIV_32;
pub const RCC_HCLK_DIV_64: u32 = RCC_CR0_HCLK_DIV_64;
pub const RCC_HCLK_DIV_128: u32 = RCC_CR0_HCLK_DIV_128;
pub const RCC_HCLK_DIV_256: u32 = RCC_CR0_HCLK_DIV_256;
pub const RCC_HCLK_DIV_512: u32 = RCC_CR0_HCLK_DIV_512;

/// The divider of the PCLK0
pub const RCC_PCLK0_DIV_1: u32 = RCC_CR0_PCLK0_DIV_1;
pub const RCC_PCLK0_DIV_2: u32 = RCC_CR0_PCLK0_DIV_2;
pub const RCC_PCLK0_DIV_4: u32 = RCC_CR0_PCLK0_DIV_4;
pub const RCC_PCLK0_DIV_8: u32 = RCC_CR0_PCLK0_DIV_8;
pub const RCC_PCLK0_DIV_16: u32 = RCC_CR0_PCLK0_DIV_16;

/// The divider of the PCLK1
pub const RCC_PCLK1_DIV_1: u32 = RCC_CR0_PCLK1_DIV_1;
pub const RCC_PCLK1_DIV_2: u32 = RCC_CR0_PCLK1_DIV_2;
pub const RCC_PCLK1_DIV_4: u32 = RCC_CR0_PCLK1_DIV_4;
pub const RCC_PCLK1_DIV_8: u32 = RCC_CR0_PCLK1_DIV_8;
pub const RCC_PCLK1_DIV_16: u32 = RCC_CR0_PCLK1_DIV_16;

/// The divider of the MCO CLK
pub const RCC_MCO_CLK_DIV_1: u32 = RCC_CR0_MCO_CLK_DIV_1;
pub const RCC_MCO_CLK_DIV_2: u32 = RCC_CR0_MCO_CLK_DIV_2;
pub const RCC_MCO_CLK_DIV_4: u32 = RCC_CR0_MCO_CLK_DIV_4;
pub const RCC_MCO_CLK_DIV_8: u32 = RCC_CR0_MCO_CLK_DIV_8;
pub const RCC_MCO_CLK_DIV_16: u32 = RCC_CR0_MCO_CLK_DIV_16;

/// RCC peripherals
pub const RCC_PERIPHERAL_SAC: u32 = 0x00;
pub const RCC_PERIPHERAL_SEC: u32 = 0x01;
pub const RCC_PERIPHERAL_CRC: u32 = 0x02;
pub const RCC_PERIPHERAL_RTC: u32 = 0x03;
pub const RCC_PERIPHERAL_WDG: u32 = 0x04;
pub const RCC_PERIPHERAL_IWDG: u32 = 0x05;
pub const RCC_PERIPHERAL_LPTIMER0: u32 = 0x06;
pub const RCC_PERIPHERAL_BSTIMER1: u32 = 0x07;
pub const RCC_PERIPHERAL_BSTIMER0: u32 = 0x08;
pub const RCC_PERIPHERAL_TIMER3: u32 = 0x09;
pub const RCC_PERIPHERAL_TIMER2: u32 = 0x0A;
pub const RCC_PERIPHERAL_TIMER1: u32 = 0x0B;
pub const RCC_PERIPHERAL_TIMER0: u32 = 0x0C;
pub const RCC_PERIPHERAL_GPIOA: u32 = 0x0D;
pub const RCC_PERIPHERAL_GPIOB: u32 = 0x20;
pub const RCC_PERIPHERAL_GPIOC: u32 = 0x21;
pub const RCC_PERIPHERAL_GPIOD: u32 = 0x22;
pub const RCC_PERIPHERAL_LORA: u32 = 0x0E;
pub const RCC_PERIPHERAL_DAC: u32 = 0x0F;
pub const RCC_PERIPHERAL_LCD: u32 = 0x10;
pub const RCC_PERIPHERAL_AFEC: u32 = 0x11;
pub const RCC_PERIPHERAL_ADC: u32 = 0x12;
pub const RCC_PERIPHERAL_I2C2: u32 = 0x14;
pub const RCC_PERIPHERAL_I2C1: u32 = 0x15;
pub const RCC_PERIPHERAL_I2C0: u32 = 0x16;
pub const RCC_PERIPHERAL_QSPI: u32 = 0x17;
pub const RCC_PERIPHERAL_SSP2: u32 = 0x18;
pub const RCC_PERIPHERAL_SSP1: u32 = 0x19;
pub const RCC_PERIPHERAL_SSP0: u32 = 0x1A;
pub const RCC_PERIPHERAL_LPUART: u32 = 0x1B;
pub const RCC_PERIPHERAL_UART3: u32 = 0x1C;
pub const RCC_PERIPHERAL_UART2: u32 = 0x1D;
pub const RCC_PERIPHERAL_UART1: u32 = 0x1E;
pub const RCC_PERIPHERAL_UART0: u32 = 0x1F;
pub const RCC_PERIPHERAL_DMA1: u32 = 0x30;
pub const RCC_PERIPHERAL_DMA0: u32 = 0x31;
pub const RCC_PERIPHERAL_I2S: u32 = 0x32;
pub const RCC_PERIPHERAL_RNGC: u32 = 0x33;
pub const RCC_PERIPHERAL_LPTIMER1: u32 = 0x34;
pub const RCC_PERIPHERAL_SYSCFG: u32 = 0x40;
pub const RCC_PERIPHERAL_PWR: u32 = 0x41;

/// RCC reset mask
pub const RCC_RESET_MASK_SEC: u32 = 0x02;
pub const RCC_RESET_MASK_CPU: u32 = 0x04;
pub const RCC_RESET_MASK_EFC: u32 = 0x08;
pub const RCC_RESET_MASK_WDG: u32 = 0x10;
pub const RCC_RESET_MASK_IWDG: u32 = 0x20;
pub const RCC_RESET_MASK_ALL: u32 = 0x3E;

impl Rcc {
    /// Get the frequency of the specified clock
    pub fn get_clk_freq(&self, clk: u32) -> u32 {
        let mut freq;
        let mut tmp = self.cr0 & RCC_CR0_SYSCLK_SEL_MASK;
        let sysclk_freq = match tmp {
            RCC_CR0_SYSCLK_SEL_RCO48M => RCC_FREQ_48M,
            RCC_CR0_SYSCLK_SEL_RCO32K => RCC_FREQ_32000,
            RCC_CR0_SYSCLK_SEL_XO32K => RCC_FREQ_32768,
            RCC_CR0_SYSCLK_SEL_XO24M => RCC_FREQ_24M,
            RCC_CR0_SYSCLK_SEL_XO32M => RCC_FREQ_32M,
            RCC_CR0_SYSCLK_SEL_RCO4M => RCC_FREQ_4M,
            _ => RCC_FREQ_24M,
        };

        match clk {
            RCC_HCLK => {
                tmp = self.cr0 & RCC_CR0_HCLK_DIV_MASK;
                tmp >>= 8;
                freq = sysclk_freq >> tmp;
            }
            RCC_PCLK0 => {
                let mut tmp = self.cr0 & RCC_CR0_HCLK_DIV_MASK;
                tmp >>= 8;
                freq = sysclk_freq >> tmp;

                tmp = self.cr0 & RCC_CR0_PCLK0_DIV_MASK;
                tmp >>= 5;
                freq >>= tmp;
            }
            RCC_PCLK1 => {
                let mut tmp = self.cr0 & RCC_CR0_HCLK_DIV_MASK;
                tmp >>= 8;
                freq = sysclk_freq >> tmp;

                tmp = self.cr0 & RCC_CR0_PCLK1_DIV_MASK;
                tmp >>= 15;
                freq >>= tmp;
            }
            _ => {
                freq = sysclk_freq;
            }
        }

        freq
    }

    /// Enable/Disable the specified oscillator
    pub fn enable_oscillator(&mut self, osc: u32, new_state: bool) {
        match osc {
            RCC_OSC_RCO48M => {
                let value = analog_read!(0x06);
                if new_state {
                    analog_write!(0x06, value & !(1 << 5));
                    while (AFEC.clone().raw_sr & AFEC_RAW_SR_RCO24M_READY_MASK) == 0 {}
                } else {
                    analog_write!(0x06, value | (1 << 5));
                    while (AFEC.clone().raw_sr & AFEC_RAW_SR_RCO24M_READY_MASK) != 0 {}
                }
            }
            RCC_OSC_RCO32K => {
                let value = analog_read!(0x02);
                analog_write!(
                    0x02,
                    if new_state {
                        value & (!(1 << 15))
                    } else {
                        value | (1 << 15)
                    }
                );
            }
            RCC_OSC_XO32K => {
                let value = analog_read!(0x02);
                analog_write!(
                    0x02,
                    if new_state {
                        value & (!(1 << 13)) & (!(1 << 14))
                    } else {
                        value | (1 << 13) | (1 << 14)
                    }
                );
            }
            RCC_OSC_XO24M => {
                let value = analog_read!(0x06);
                analog_write!(
                    0x06,
                    if new_state {
                        (value | (1 << 3)) & (!(1 << 4))
                    } else {
                        (value & (!(1 << 3))) | (1 << 4)
                    }
                );
            }
            RCC_OSC_XO32M => {
                self.enable_peripheral_clk(RCC_PERIPHERAL_LORA, true);
                let lorac = &mut LORAC.clone();
                if new_state {
                    if (lorac.cr1 & 0x00000020) == 0 {
                        lorac.cr1 |= 1 << 5; // nreset
                        lorac.cr1 &= !(1 << 7); // por
                    }

                    lorac.cr1 |= 1 << 2;
                    while (LORAC.clone().sr & (1 << 1)) == 0 {}
                } else {
                    lorac.cr1 &= !(1 << 2);
                    while (LORAC.clone().sr & (1 << 1)) != 0 {}
                }
            }
            RCC_OSC_RCO4M => {
                let value = analog_read!(0x06);
                if new_state {
                    analog_write!(0x06, value & !(1 << 6));
                    while (AFEC.clone().raw_sr & AFEC_RAW_SR_RCO4M_READY_MASK) == 0 {}
                } else {
                    analog_write!(0x06, value | (1 << 6));
                    while (AFEC.clone().raw_sr & AFEC_RAW_SR_RCO4M_READY_MASK) != 0 {}
                }
            }
            _ => {}
        }
    }

    /// Set the source of the SYSCLK
    pub fn set_sys_clk_src(&mut self, clk_src: u32) {
        set_reg_bits!(self, cr0, RCC_CR0_SYSCLK_SEL_MASK, clk_src);
    }

    /// Set the source of the SYSTICK
    pub fn set_systick_src(&mut self, clk_src: u32) {
        if clk_src == RCC_SYSTICK_SOURCE_HCLK {
            todo!("SYSTICK")
            // tremo_reg_en!(SYSTICK, ctrl, SysTick_CTRL_CLKSOURCE_Msk, true);
        } else {
            toggle_reg_bits!(self, cr0, RCC_CR0_HCLK_DIV_MASK, false);
        }
    }

    /// Set the source of the MCO clock
    pub fn set_mco_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_MCO_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cr0, RCC_CR0_MCO_CLK_SEL_MASK, false);
            while self.clone().sr1 & RCC_SR1_MCO_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr0, RCC_CR0_MCO_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the UART0 CLK
    pub fn set_uart0_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_UART0_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr0, RCC_CGR0_UART0_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_UART0_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr2, RCC_CR2_UART0_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the UART1 CLK
    pub fn set_uart1_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_UART1_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr0, RCC_CGR0_UART1_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_UART1_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr2, RCC_CR2_UART1_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the UART2 CLK
    pub fn set_uart2_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_UART2_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr0, RCC_CGR0_UART2_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_UART2_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr2, RCC_CR2_UART2_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the UART3 CLK
    pub fn set_uart3_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_UART3_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr0, RCC_CGR0_UART3_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_UART3_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr2, RCC_CR2_UART3_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the LPTIMER0 CLK
    pub fn set_lptimer0_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_LPTIMER0_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER0_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_LPTIMER0_CLK_EN_SYNC != 0 {}
        }

        if clk_src == RCC_LPTIMER0_CLK_SOURCE_EXTCLK {
            toggle_reg_bits!(self, cr1, RCC_CR1_LPTIMER0_EXTCLK_SEL_MASK, true);
        } else {
            set_reg_bits!(self, cr1, RCC_CR1_LPTIMER0_CLK_SEL_MASK, clk_src);
        }
    }

    /// Set the source of the LPTIMER1 CLK
    pub fn set_lptimer1_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_LPTIMER1_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER1_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_LPTIMER1_CLK_EN_SYNC != 0 {}
        }

        if clk_src == RCC_LPTIMER1_CLK_SOURCE_EXTCLK {
            toggle_reg_bits!(self, cr1, RCC_CR1_LPTIMER1_EXTCLK_SEL_MASK, true);
        } else {
            set_reg_bits!(self, cr1, RCC_CR1_LPTIMER1_CLK_SEL_MASK, clk_src);
        }
    }

    /// Set the source of the LCD CLK
    pub fn set_lcd_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_LCD_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr0, RCC_CGR0_LCD_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_LCD_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr1, RCC_CR1_LCD_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the LPUART CLK
    pub fn set_lpuart_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_LPUART_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr0, RCC_CGR0_LPUART_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_LPUART_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr1, RCC_CR1_LPUART_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the RTC CLK
    pub fn set_rtc_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_RTC_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr1, RCC_CGR1_RTC_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_RTC_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr1, RCC_CR1_RTC_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the IWDG CLK
    pub fn set_iwdg_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_IWDG_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr1, RCC_CGR1_IWDG_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_IWDG_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr1, RCC_CR1_IWDG_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the ADC CLK
    pub fn set_adc_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_ADC_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr0, RCC_CGR0_ADC_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_ADC_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr2, RCC_CR2_ADC_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the QSPI CLK
    pub fn set_qspi_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_QSPI_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr1, RCC_CGR1_QSPI_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_QSPI_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr2, RCC_CR2_QSPI_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the I2S CLK
    pub fn set_i2s_clk_src(&mut self, clk_src: u32) {
        if self.sr1 & RCC_SR1_I2S_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cgr1, RCC_CGR1_I2S_CLK_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_I2S_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr2, RCC_CR2_I2S_CLK_SEL_MASK, clk_src);
    }

    /// Get the source of the SYSCLK
    pub fn get_sys_clk_src(&self) -> u32 {
        self.cr0 & RCC_CR0_SYSCLK_SEL_MASK
    }

    /// Get the source of the SYSTICK
    pub fn get_systick_src(&self) -> u32 {
        todo!("SYSTICK")
        // if SYSTICK.ctrl & SysTick_CTRL_CLKSOURCE_Msk != 0 {
        //     RCC_SYSTICK_SOURCE_HCLK
        // } else {
        //     self.cr0 & RCC_CR0_HCLK_DIV_MASK
        // }
    }

    /// Get the source of the MCO clock
    pub fn get_mco_clk_src(&self) -> u32 {
        self.cr0 & RCC_CR0_MCO_CLK_SEL_MASK
    }

    /// Get the source of the UART0 CLK
    pub fn get_uart0_clk_src(&self) -> u32 {
        self.cr2 & RCC_CR2_UART0_CLK_SEL_MASK
    }

    /// Get the source of the UART1 CLK
    pub fn get_uart1_clk_src(&self) -> u32 {
        self.cr2 & RCC_CR2_UART1_CLK_SEL_MASK
    }

    /// Get the source of the UART2 CLK
    pub fn get_uart2_clk_src(&self) -> u32 {
        self.cr2 & RCC_CR2_UART2_CLK_SEL_MASK
    }

    /// Get the source of the UART3 CLK
    pub fn get_uart3_clk_src(&self) -> u32 {
        self.cr2 & RCC_CR2_UART3_CLK_SEL_MASK
    }

    /// Get the source of the LPTIMER0 CLK
    pub fn get_lptimer0_get_clk_src(&self) -> u32 {
        if self.cr1 & RCC_CR1_LPTIMER0_EXTCLK_SEL_MASK != 0 {
            RCC_LPTIMER0_CLK_SOURCE_EXTCLK
        } else {
            self.cr1 & RCC_CR1_LPTIMER0_CLK_SEL_MASK
        }
    }

    /// Get the source of the LPTIMER1 CLK
    pub fn get_lptimer1_get_clk_src(&self) -> u32 {
        if self.cr1 & RCC_CR1_LPTIMER1_EXTCLK_SEL_MASK != 0 {
            RCC_LPTIMER1_CLK_SOURCE_EXTCLK
        } else {
            self.cr1 & RCC_CR1_LPTIMER1_CLK_SEL_MASK
        }
    }

    /// Get the source of the LCD CLK
    pub fn get_lcd_get_clk_src(&self) -> u32 {
        self.cr1 & RCC_CR1_LCD_CLK_SEL_MASK
    }

    /// Get the source of the LPUART CLK
    pub fn get_lpuart_clk_src(&self) -> u32 {
        self.cr1 & RCC_CR1_LPUART_CLK_SEL_MASK
    }

    /// Get the source of the RTC CLK
    pub fn get_rtc_clk_src(&self) -> u32 {
        self.cr1 & RCC_CR1_RTC_CLK_SEL_MASK
    }

    /// Get the source of the IWDG CLK
    pub fn get_iwdg_clk_src(&self) -> u32 {
        self.cr1 & RCC_CR1_IWDG_CLK_SEL_MASK
    }

    /// Get the source of the ADC CLK
    pub fn get_adc_clk_src(&self) -> u32 {
        self.cr2 & RCC_CR2_ADC_CLK_SEL_MASK
    }

    /// Get the source of the QSPI CLK
    pub fn get_qspi_clk_src(&self) -> u32 {
        self.cr2 & RCC_CR2_QSPI_CLK_SEL_MASK
    }

    /// Get the source of the I2S CLK
    pub fn get_i2s_clk_src(&self) -> u32 {
        self.cr2 & RCC_CR2_I2S_CLK_SEL_MASK
    }

    /// Set the divider of the HCLK
    pub fn set_hclk_div(&mut self, div: u32) {
        set_reg_bits!(self, cr0, RCC_CR0_HCLK_DIV_MASK, div);
    }

    /// Set the divider of the PCLK
    pub fn set_pclk_div(&mut self, pclk0_div: u32, pclk1_div: u32) {
        set_reg_bits!(
            self,
            cr0,
            RCC_CR0_PCLK0_DIV_MASK | RCC_CR0_PCLK1_DIV_MASK,
            pclk0_div | pclk1_div
        );
    }

    /// Set the divider of the MCO CLK
    pub fn set_mco_clk_div(&mut self, div: u32) {
        if self.sr1 & RCC_SR1_MCO_CLK_EN_SYNC != 0 {
            toggle_reg_bits!(self, cr0, RCC_CR0_MCO_CLK_OUT_EN_MASK, false);
            while self.clone().sr1 & RCC_SR1_MCO_CLK_EN_SYNC != 0 {}
        }
        set_reg_bits!(self, cr0, RCC_CR0_MCO_CLK_DIV_MASK, div);
    }

    /// Enable/Disable the clock of the specified peripheral
    pub fn enable_peripheral_clk(&mut self, peripheral: u32, new_state: bool) {
        match peripheral {
            RCC_PERIPHERAL_UART0 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_UART0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_UART1 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_UART1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_UART2 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_UART2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_UART3 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_UART3_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_LPUART => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_LPUART_CLK_EN_MASK, new_state);
                while (self.clone().sr & RCC_SR_ALL_DONE) != RCC_SR_ALL_DONE {}
                toggle_reg_bits!(self, cgr2, RCC_CGR2_LPUART_AON_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SSP0 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_SSP0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SSP1 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_SSP1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SSP2 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_SSP2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_QSPI => {
                toggle_reg_bits!(self, cgr1, RCC_CGR1_QSPI_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_I2C0 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_I2C0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_I2C1 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_I2C1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_I2C2 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_I2C2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_ADC => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_ADC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_DAC => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_DAC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_AFEC => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_AFEC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_LCD => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_LCD_CLK_EN_MASK, new_state);
                while (self.clone().sr & RCC_SR_ALL_DONE) != RCC_SR_ALL_DONE {}
                toggle_reg_bits!(self, cgr2, RCC_CGR2_LCD_AON_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_LORA => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_LORA_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_GPIOA => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_IOM0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_GPIOB => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_IOM1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_GPIOC => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_IOM2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_GPIOD => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_IOM3_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_TIMER0 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_TIMER0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_TIMER1 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_TIMER1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_TIMER2 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_TIMER2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_TIMER3 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_TIMER3_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_BSTIMER0 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_BSTIMER0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_BSTIMER1 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_BSTIMER1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_LPTIMER0 => {
                if new_state {
                    toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER0_PCLK_EN_MASK, new_state);
                    while (self.clone().sr & RCC_SR_ALL_DONE) != RCC_SR_ALL_DONE {}
                    toggle_reg_bits!(self, cgr2, RCC_CGR2_LPTIMER0_AON_CLK_EN_MASK, new_state);
                    toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER0_CLK_EN_MASK, new_state);
                } else {
                    toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER0_CLK_EN_MASK, new_state);
                    while (self.clone().sr & RCC_SR_ALL_DONE) != RCC_SR_ALL_DONE {}
                    toggle_reg_bits!(self, cgr2, RCC_CGR2_LPTIMER0_AON_CLK_EN_MASK, new_state);
                    toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER0_PCLK_EN_MASK, new_state);
                }
            }
            RCC_PERIPHERAL_LPTIMER1 => {
                if new_state {
                    toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER1_PCLK_EN_MASK, new_state);
                    while (self.clone().sr & RCC_SR_ALL_DONE) != RCC_SR_ALL_DONE {}
                    toggle_reg_bits!(self, cgr2, RCC_CGR2_LPTIMER1_AON_CLK_EN_MASK, new_state);
                    toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER1_CLK_EN_MASK, new_state);
                } else {
                    toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER1_CLK_EN_MASK, new_state);
                    while (self.clone().sr & RCC_SR_ALL_DONE) != RCC_SR_ALL_DONE {}
                    toggle_reg_bits!(self, cgr2, RCC_CGR2_LPTIMER1_AON_CLK_EN_MASK, new_state);
                    toggle_reg_bits!(self, cgr1, RCC_CGR1_LPTIMER1_PCLK_EN_MASK, new_state);
                }
            }
            RCC_PERIPHERAL_IWDG => {
                toggle_reg_bits!(self, cgr1, RCC_CGR1_IWDG_CLK_EN_MASK, new_state);
                while (self.clone().sr & RCC_SR_ALL_DONE) != RCC_SR_ALL_DONE {}
                toggle_reg_bits!(self, cgr2, RCC_CGR2_IWDG_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_WDG => {
                toggle_reg_bits!(self, cgr1, RCC_CGR1_WDG_CLK_EN_MASK, new_state);
                toggle_reg_bits!(self, cgr1, RCC_CGR1_WDG_CNT_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_RTC => {
                toggle_reg_bits!(self, cgr1, RCC_CGR1_RTC_CLK_EN_MASK, new_state);
                while (self.clone().sr & RCC_SR_ALL_DONE) != RCC_SR_ALL_DONE {}
                toggle_reg_bits!(self, cgr2, RCC_CGR2_RTC_AON_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_CRC => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_CRC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SEC => {
                toggle_reg_bits!(self, cgr1, RCC_CGR1_SEC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SAC => {
                toggle_reg_bits!(self, cgr1, RCC_CGR1_SAC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_I2S => {
                toggle_reg_bits!(self, cgr1, RCC_CGR1_I2S_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_DMA0 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_DMAC0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_DMA1 => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_DMAC1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_RNGC => {
                toggle_reg_bits!(self, cgr1, RCC_CGR1_RNGC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SYSCFG => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_SYSCFG_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_PWR => {
                toggle_reg_bits!(self, cgr0, RCC_CGR0_PWR_CLK_EN_MASK, new_state);
            }
            _ => {}
        }
    }

    /// Enable/Disable the output of the mco clk
    pub fn enable_mco_clk_output(&mut self, new_state: bool) {
        toggle_reg_bits!(self, cr0, RCC_CR0_MCO_CLK_OUT_EN_MASK, new_state);
    }

    /// Reset the register of the specified peripheral to the reset value
    pub fn rst_peripheral(&mut self, mut peripheral: u32, new_state: bool) {
        if peripheral >= RCC_PERIPHERAL_SYSCFG {
            return;
        }

        if peripheral >= RCC_PERIPHERAL_DMA1 {
            toggle_reg_bits!(
                self,
                rst1,
                1 << (peripheral - RCC_PERIPHERAL_DMA1),
                !new_state
            );
        } else {
            if matches!(
                peripheral,
                RCC_PERIPHERAL_GPIOB | RCC_PERIPHERAL_GPIOC | RCC_PERIPHERAL_GPIOD
            ) {
                peripheral = RCC_PERIPHERAL_GPIOA;
            }

            toggle_reg_bits!(self, rst0, 1 << peripheral, !new_state);
        }
    }

    /// Set the reset mask
    pub fn set_reset_mask(&mut self, mask: u32) {
        set_reg_bits!(self, rst_cr, RCC_RST_CR_RESET_REQ_EN_MASK, mask);
    }

    /// Get the reset mask
    pub fn get_reset_mask(&self) -> u32 {
        self.rst_cr & RCC_RST_CR_RESET_REQ_EN_MASK
    }

    /// Set the divider of the I2S MCLK
    pub fn set_i2s_mclk_div(&mut self, div: u32) {
        set_reg_bits!(self, cr3, RCC_CR3_I2S_MCLK_DIV_MASK, div << 8);
    }

    /// Set the divider of the I2S SCLK
    pub fn set_i2s_sclk_div(&mut self, div: u32) {
        set_reg_bits!(self, cr3, RCC_CR3_I2S_SCLK_DIV_MASK, div);
    }
}
