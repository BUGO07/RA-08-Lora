use crate::{
    ffi::{
        AFEC_RAW_SR_RCO4M_READY_MASK, AFEC_RAW_SR_RCO24M_READY_MASK, RCC_CR0_HCLK_DIV_MASK,
        RCC_CR0_PCLK0_DIV_MASK, RCC_CR0_PCLK1_DIV_MASK, RCC_CR0_SYSCLK_SEL_MASK,
        RCC_CR0_SYSCLK_SEL_RCO4M, RCC_CR0_SYSCLK_SEL_RCO32K, RCC_CR0_SYSCLK_SEL_RCO48M,
        RCC_CR0_SYSCLK_SEL_XO24M, RCC_CR0_SYSCLK_SEL_XO32K, RCC_CR0_SYSCLK_SEL_XO32M,
        RCC_CR2_UART0_CLK_SEL_MASK, RCC_FREQ_4M, RCC_FREQ_24M, RCC_FREQ_32M, RCC_FREQ_48M,
        RCC_FREQ_32000, RCC_FREQ_32768, RCC_HCLK, RCC_OSC_RCO4M, RCC_OSC_RCO32K, RCC_OSC_RCO48M,
        RCC_OSC_XO24M, RCC_OSC_XO32K, RCC_OSC_XO32M, RCC_PCLK0, RCC_PCLK1, RCC_PERIPHERAL_LORA,
        RCC_PERIPHERAL_UART0, RCC_PERIPHERAL_UART1, RCC_PERIPHERAL_UART2, RCC_PERIPHERAL_UART3, *,
    },
    peripherals::regs::{AFEC, LORAC, RCC, Rcc},
    tremo_analog_rd, tremo_analog_wr, tremo_reg_en, tremo_reg_rd, tremo_reg_set,
};

impl Rcc {
    /// Get the frequency of the specified clock
    pub fn get_clk_freq(&self, clk: u32) -> u32 {
        let rcc = unsafe { &*self.0 };

        let mut freq;
        let mut tmp = rcc.cr0 & RCC_CR0_SYSCLK_SEL_MASK as u32;
        let sysclk_freq = match tmp as u16 {
            RCC_CR0_SYSCLK_SEL_RCO48M => RCC_FREQ_48M,
            RCC_CR0_SYSCLK_SEL_RCO32K => RCC_FREQ_32000 as u32,
            RCC_CR0_SYSCLK_SEL_XO32K => RCC_FREQ_32768 as u32,
            RCC_CR0_SYSCLK_SEL_XO24M => RCC_FREQ_24M,
            RCC_CR0_SYSCLK_SEL_XO32M => RCC_FREQ_32M,
            RCC_CR0_SYSCLK_SEL_RCO4M => RCC_FREQ_4M,
            _ => RCC_FREQ_24M,
        };

        match clk {
            RCC_HCLK => {
                tmp = rcc.cr0 & RCC_CR0_HCLK_DIV_MASK as u32;
                tmp >>= 8;
                freq = sysclk_freq >> tmp;
            }
            RCC_PCLK0 => {
                let mut tmp = rcc.cr0 & RCC_CR0_HCLK_DIV_MASK as u32;
                tmp >>= 8;
                freq = sysclk_freq >> tmp;

                tmp = rcc.cr0 & RCC_CR0_PCLK0_DIV_MASK as u32;
                tmp >>= 5;
                freq >>= tmp;
            }
            RCC_PCLK1 => {
                let mut tmp = rcc.cr0 & RCC_CR0_HCLK_DIV_MASK as u32;
                tmp >>= 8;
                freq = sysclk_freq >> tmp;

                tmp = rcc.cr0 & RCC_CR0_PCLK1_DIV_MASK;
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
    pub fn enable_oscillator(&self, osc: u32, new_state: bool) {
        match osc {
            RCC_OSC_RCO48M => {
                let value = tremo_analog_rd!(0x06);
                if new_state {
                    tremo_analog_wr!(0x06, value & !(1 << 5));
                    while ((unsafe { &*AFEC.0 }).raw_sr & AFEC_RAW_SR_RCO24M_READY_MASK as u32) == 0
                    {
                    }
                } else {
                    tremo_analog_wr!(0x06, value | (1 << 5));
                    while ((unsafe { &*AFEC.0 }).raw_sr & AFEC_RAW_SR_RCO24M_READY_MASK as u32) != 0
                    {
                    }
                }
            }
            RCC_OSC_RCO32K => {
                let value = tremo_analog_rd!(0x02);
                tremo_analog_wr!(
                    0x02,
                    if new_state {
                        value & (!(1 << 15))
                    } else {
                        value | (1 << 15)
                    }
                );
            }
            RCC_OSC_XO32K => {
                let value = tremo_analog_rd!(0x02);
                tremo_analog_wr!(
                    0x02,
                    if new_state {
                        value & (!(1 << 13)) & (!(1 << 14))
                    } else {
                        value | (1 << 13) | (1 << 14)
                    }
                );
            }
            RCC_OSC_XO24M => {
                let value = tremo_analog_rd!(0x06);
                tremo_analog_wr!(
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
                if new_state {
                    let lora = unsafe { &mut *LORAC.0 };
                    if (lora.cr1 & 0x00000020) == 0 {
                        lora.cr1 |= 1 << 5; // nreset
                        lora.cr1 &= !(1 << 7); // por
                    }

                    lora.cr1 |= 1 << 2;
                    while (unsafe { &*LORAC.0 }.sr & (1 << 1)) == 0 {}
                } else {
                    let lora = unsafe { &mut *LORAC.0 };
                    lora.cr1 &= !(1 << 2);
                    while (unsafe { &*LORAC.0 }.sr & (1 << 1)) != 0 {}
                }
            }
            RCC_OSC_RCO4M => {
                let value = tremo_analog_rd!(0x06);
                if new_state {
                    tremo_analog_wr!(0x06, value & !(1 << 6));
                    while ((unsafe { &*AFEC.0 }).raw_sr & AFEC_RAW_SR_RCO4M_READY_MASK) == 0 {}
                } else {
                    tremo_analog_wr!(0x06, value | (1 << 6));
                    while ((unsafe { &*AFEC.0 }).raw_sr & AFEC_RAW_SR_RCO4M_READY_MASK) != 0 {}
                }
            }
            _ => {}
        }
    }

    /// Set the source of the SYSCLK
    pub fn set_sys_clk_src(&self, clk_src: u32) {
        tremo_reg_set!(self, cr0, RCC_CR0_SYSCLK_SEL_MASK, clk_src);
    }

    /// Set the source of the SYSTICK
    pub fn set_systick_src(&self, clk_src: u32) {
        if clk_src == RCC_SYSTICK_SOURCE_HCLK {
            todo!("SYSTICK")
            // tremo_reg_en!(SYSTICK, ctrl, SysTick_CTRL_CLKSOURCE_Msk, true);
        } else {
            tremo_reg_en!(self, cr0, RCC_CR0_HCLK_DIV_MASK, false);
        }
    }

    /// Set the source of the MCO clock
    pub fn set_mco_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_MCO_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cr0, RCC_CR0_MCO_CLK_SEL_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_MCO_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr0, RCC_CR0_MCO_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the UART0 CLK
    pub fn set_uart0_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_UART0_CLK_EN_SYNC != 0 {
            tremo_reg_en!(self, cgr0, RCC_CGR0_UART0_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_UART0_CLK_EN_SYNC != 0 {}
        }
        tremo_reg_set!(self, cr2, RCC_CR2_UART0_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the UART1 CLK
    pub fn set_uart1_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_UART1_CLK_EN_SYNC != 0 {
            tremo_reg_en!(self, cgr0, RCC_CGR0_UART1_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_UART1_CLK_EN_SYNC != 0 {}
        }
        tremo_reg_set!(self, cr2, RCC_CR2_UART1_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the UART2 CLK
    pub fn set_uart2_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_UART2_CLK_EN_SYNC != 0 {
            tremo_reg_en!(self, cgr0, RCC_CGR0_UART2_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_UART2_CLK_EN_SYNC != 0 {}
        }
        tremo_reg_set!(self, cr2, RCC_CR2_UART2_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the UART3 CLK
    pub fn set_uart3_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_UART3_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cgr0, RCC_CGR0_UART3_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_UART3_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr2, RCC_CR2_UART3_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the LPTIMER0 CLK
    pub fn set_lptimer0_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_LPTIMER0_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cgr1, RCC_CGR1_LPTIMER0_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_LPTIMER0_CLK_EN_SYNC as u32 != 0 {}
        }

        if clk_src == RCC_LPTIMER0_CLK_SOURCE_EXTCLK {
            tremo_reg_en!(self, cr1, RCC_CR1_LPTIMER0_EXTCLK_SEL_MASK, true);
        } else {
            tremo_reg_set!(self, cr1, RCC_CR1_LPTIMER0_CLK_SEL_MASK, clk_src);
        }
    }

    /// Set the source of the LPTIMER1 CLK
    pub fn set_lptimer1_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_LPTIMER1_CLK_EN_SYNC != 0 {
            tremo_reg_en!(self, cgr1, RCC_CGR1_LPTIMER1_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_LPTIMER1_CLK_EN_SYNC != 0 {}
        }

        if clk_src == RCC_LPTIMER1_CLK_SOURCE_EXTCLK {
            tremo_reg_en!(self, cr1, RCC_CR1_LPTIMER1_EXTCLK_SEL_MASK, true);
        } else {
            tremo_reg_set!(self, cr1, RCC_CR1_LPTIMER1_CLK_SEL_MASK, clk_src);
        }
    }

    /// Set the source of the LCD CLK
    pub fn set_lcd_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_LCD_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cgr0, RCC_CGR0_LCD_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_LCD_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr1, RCC_CR1_LCD_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the LPUART CLK
    pub fn set_lpuart_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_LPUART_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cgr0, RCC_CGR0_LPUART_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_LPUART_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr1, RCC_CR1_LPUART_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the RTC CLK
    pub fn set_rtc_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_RTC_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cgr1, RCC_CGR1_RTC_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_RTC_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr1, RCC_CR1_RTC_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the IWDG CLK
    pub fn set_iwdg_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_IWDG_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cgr1, RCC_CGR1_IWDG_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_IWDG_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr1, RCC_CR1_IWDG_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the ADC CLK
    pub fn set_adc_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_ADC_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cgr0, RCC_CGR0_ADC_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_ADC_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr2, RCC_CR2_ADC_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the QSPI CLK
    pub fn set_qspi_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_QSPI_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cgr1, RCC_CGR1_QSPI_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_QSPI_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr2, RCC_CR2_QSPI_CLK_SEL_MASK, clk_src);
    }

    /// Set the source of the I2S CLK
    pub fn set_i2s_clk_src(&self, clk_src: u32) {
        if tremo_reg_rd!(RCC, sr1) & RCC_SR1_I2S_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cgr1, RCC_CGR1_I2S_CLK_EN_MASK, false);
            while tremo_reg_rd!(RCC, sr1) & RCC_SR1_I2S_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr2, RCC_CR2_I2S_CLK_SEL_MASK, clk_src);
    }

    /// Get the source of the SYSCLK
    pub fn get_sys_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr0) & RCC_CR0_SYSCLK_SEL_MASK as u32
    }

    /// Get the source of the SYSTICK
    pub fn get_systick_src(&self) -> u32 {
        todo!("SYSTICK")
        // if tremo_reg_rd!(SYSTICK, ctrl) & SysTick_CTRL_CLKSOURCE_Msk != 0 {
        //     RCC_SYSTICK_SOURCE_HCLK
        // } else {
        //     tremo_reg_rd!(self, cr0) & RCC_CR0_HCLK_DIV_MASK as u32
        // }
    }

    /// Get the source of the MCO clock
    pub fn get_mco_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr0) & RCC_CR0_MCO_CLK_SEL_MASK
    }

    /// Get the source of the UART0 CLK
    pub fn get_uart0_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr2) & RCC_CR2_UART0_CLK_SEL_MASK
    }

    /// Get the source of the UART1 CLK
    pub fn get_uart1_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr2) & RCC_CR2_UART1_CLK_SEL_MASK as u32
    }

    /// Get the source of the UART2 CLK
    pub fn get_uart2_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr2) & RCC_CR2_UART2_CLK_SEL_MASK as u32
    }

    /// Get the source of the UART3 CLK
    pub fn get_uart3_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr2) & RCC_CR2_UART3_CLK_SEL_MASK as u32
    }

    /// Get the source of the LPTIMER0 CLK
    pub fn get_lptimer0_get_clk_src(&self) -> u32 {
        if tremo_reg_rd!(self, cr1) & RCC_CR1_LPTIMER0_EXTCLK_SEL_MASK as u32 != 0 {
            RCC_LPTIMER0_CLK_SOURCE_EXTCLK
        } else {
            tremo_reg_rd!(self, cr1) & RCC_CR1_LPTIMER0_CLK_SEL_MASK as u32
        }
    }

    /// Get the source of the LPTIMER1 CLK
    pub fn get_lptimer1_get_clk_src(&self) -> u32 {
        if tremo_reg_rd!(self, cr1) & RCC_CR1_LPTIMER1_EXTCLK_SEL_MASK as u32 != 0 {
            RCC_LPTIMER1_CLK_SOURCE_EXTCLK
        } else {
            tremo_reg_rd!(self, cr1) & RCC_CR1_LPTIMER1_CLK_SEL_MASK as u32
        }
    }

    /// Get the source of the LCD CLK
    pub fn get_lcd_get_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr1) & RCC_CR1_LCD_CLK_SEL_MASK as u32
    }

    /// Get the source of the LPUART CLK
    pub fn get_lpuart_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr1) & RCC_CR1_LPUART_CLK_SEL_MASK as u32
    }

    /// Get the source of the RTC CLK
    pub fn get_rtc_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr1) & RCC_CR1_RTC_CLK_SEL_MASK as u32
    }

    /// Get the source of the IWDG CLK
    pub fn get_iwdg_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr1) & RCC_CR1_IWDG_CLK_SEL_MASK as u32
    }

    /// Get the source of the ADC CLK
    pub fn get_adc_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr2) & RCC_CR2_ADC_CLK_SEL_MASK as u32
    }

    /// Get the source of the QSPI CLK
    pub fn get_qspi_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr2) & RCC_CR2_QSPI_CLK_SEL_MASK as u32
    }

    /// Get the source of the I2S CLK
    pub fn get_i2s_clk_src(&self) -> u32 {
        tremo_reg_rd!(self, cr2) & RCC_CR2_I2S_CLK_SEL_MASK as u32
    }

    /// Set the divider of the HCLK
    pub fn set_hclk_div(&self, div: u32) {
        tremo_reg_set!(self, cr0, RCC_CR0_HCLK_DIV_MASK, div);
    }

    /// Set the divider of the PCLK
    pub fn set_pclk_div(&self, pclk0_div: u32, pclk1_div: u32) {
        tremo_reg_set!(
            self,
            cr0,
            RCC_CR0_PCLK0_DIV_MASK as u32 | RCC_CR0_PCLK1_DIV_MASK,
            pclk0_div | pclk1_div
        );
    }

    /// Set the divider of the MCO CLK
    pub fn set_mco_clk_div(&self, div: u32) {
        if tremo_reg_rd!(self, sr1) & RCC_SR1_MCO_CLK_EN_SYNC as u32 != 0 {
            tremo_reg_en!(self, cr0, RCC_CR0_MCO_CLK_OUT_EN_MASK, false);
            while tremo_reg_rd!(self, sr1) & RCC_SR1_MCO_CLK_EN_SYNC as u32 != 0 {}
        }
        tremo_reg_set!(self, cr0, RCC_CR0_MCO_CLK_DIV_MASK, div);
    }

    /// Enable/Disable the clock of the specified peripheral
    pub fn enable_peripheral_clk(&self, peripheral: u32, new_state: bool) {
        match peripheral {
            RCC_PERIPHERAL_UART0 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_UART0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_UART1 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_UART1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_UART2 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_UART2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_UART3 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_UART3_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_LPUART => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_LPUART_CLK_EN_MASK, new_state);
                while ((unsafe { &*RCC.0 }).sr & RCC_SR_ALL_DONE as u32) != RCC_SR_ALL_DONE as u32 {
                }
                tremo_reg_en!(RCC, cgr2, RCC_CGR2_LPUART_AON_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SSP0 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_SSP0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SSP1 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_SSP1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SSP2 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_SSP2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_QSPI => {
                tremo_reg_en!(RCC, cgr1, RCC_CGR1_QSPI_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_I2C0 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_I2C0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_I2C1 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_I2C1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_I2C2 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_I2C2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_ADC => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_ADC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_DAC => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_DAC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_AFEC => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_AFEC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_LCD => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_LCD_CLK_EN_MASK, new_state);
                while ((unsafe { &*RCC.0 }).sr & RCC_SR_ALL_DONE as u32) != RCC_SR_ALL_DONE as u32 {
                }
                tremo_reg_en!(RCC, cgr2, RCC_CGR2_LCD_AON_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_LORA => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_LORA_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_GPIOA => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_IOM0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_GPIOB => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_IOM1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_GPIOC => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_IOM2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_GPIOD => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_IOM3_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_TIMER0 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_TIMER0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_TIMER1 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_TIMER1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_TIMER2 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_TIMER2_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_TIMER3 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_TIMER3_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_BSTIMER0 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_BSTIMER0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_BSTIMER1 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_BSTIMER1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_LPTIMER0 => {
                if new_state {
                    tremo_reg_en!(RCC, cgr1, RCC_CGR1_LPTIMER0_PCLK_EN_MASK, new_state);
                    while ((unsafe { &*RCC.0 }).sr & RCC_SR_ALL_DONE as u32)
                        != RCC_SR_ALL_DONE as u32
                    {}
                    tremo_reg_en!(RCC, cgr2, RCC_CGR2_LPTIMER0_AON_CLK_EN_MASK, new_state);
                    tremo_reg_en!(RCC, cgr1, RCC_CGR1_LPTIMER0_CLK_EN_MASK, new_state);
                } else {
                    tremo_reg_en!(RCC, cgr1, RCC_CGR1_LPTIMER0_CLK_EN_MASK, new_state);
                    while ((unsafe { &*RCC.0 }).sr & RCC_SR_ALL_DONE as u32)
                        != RCC_SR_ALL_DONE as u32
                    {}
                    tremo_reg_en!(RCC, cgr2, RCC_CGR2_LPTIMER0_AON_CLK_EN_MASK, new_state);
                    tremo_reg_en!(RCC, cgr1, RCC_CGR1_LPTIMER0_PCLK_EN_MASK, new_state);
                }
            }
            RCC_PERIPHERAL_LPTIMER1 => {
                if new_state {
                    tremo_reg_en!(RCC, cgr1, RCC_CGR1_LPTIMER1_PCLK_EN_MASK, new_state);
                    while ((unsafe { &*RCC.0 }).sr & RCC_SR_ALL_DONE as u32)
                        != RCC_SR_ALL_DONE as u32
                    {}
                    tremo_reg_en!(RCC, cgr2, RCC_CGR2_LPTIMER1_AON_CLK_EN_MASK, new_state);
                    tremo_reg_en!(RCC, cgr1, RCC_CGR1_LPTIMER1_CLK_EN_MASK, new_state);
                } else {
                    tremo_reg_en!(RCC, cgr1, RCC_CGR1_LPTIMER1_CLK_EN_MASK, new_state);
                    while ((unsafe { &*RCC.0 }).sr & RCC_SR_ALL_DONE as u32)
                        != RCC_SR_ALL_DONE as u32
                    {}
                    tremo_reg_en!(RCC, cgr2, RCC_CGR2_LPTIMER1_AON_CLK_EN_MASK, new_state);
                    tremo_reg_en!(RCC, cgr1, RCC_CGR1_LPTIMER1_PCLK_EN_MASK, new_state);
                }
            }
            RCC_PERIPHERAL_IWDG => {
                tremo_reg_en!(RCC, cgr1, RCC_CGR1_IWDG_CLK_EN_MASK, new_state);
                while ((unsafe { &*RCC.0 }).sr & RCC_SR_ALL_DONE as u32) != RCC_SR_ALL_DONE as u32 {
                }
                tremo_reg_en!(RCC, cgr2, RCC_CGR2_IWDG_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_WDG => {
                tremo_reg_en!(RCC, cgr1, RCC_CGR1_WDG_CLK_EN_MASK, new_state);
                tremo_reg_en!(RCC, cgr1, RCC_CGR1_WDG_CNT_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_RTC => {
                tremo_reg_en!(RCC, cgr1, RCC_CGR1_RTC_CLK_EN_MASK, new_state);
                while ((unsafe { &*RCC.0 }).sr & RCC_SR_ALL_DONE as u32) != RCC_SR_ALL_DONE as u32 {
                }
                tremo_reg_en!(RCC, cgr2, RCC_CGR2_RTC_AON_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_CRC => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_CRC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SEC => {
                tremo_reg_en!(RCC, cgr1, RCC_CGR1_SEC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SAC => {
                tremo_reg_en!(RCC, cgr1, RCC_CGR1_SAC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_I2S => {
                tremo_reg_en!(RCC, cgr1, RCC_CGR1_I2S_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_DMA0 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_DMAC0_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_DMA1 => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_DMAC1_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_RNGC => {
                tremo_reg_en!(RCC, cgr1, RCC_CGR1_RNGC_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_SYSCFG => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_SYSCFG_CLK_EN_MASK, new_state);
            }
            RCC_PERIPHERAL_PWR => {
                tremo_reg_en!(RCC, cgr0, RCC_CGR0_PWR_CLK_EN_MASK, new_state);
            }
            _ => {}
        }
    }

    /// Enable/Disable the output of the mco clk
    pub fn enable_mco_clk_output(&self, new_state: bool) {
        tremo_reg_en!(self, cr0, RCC_CR0_MCO_CLK_OUT_EN_MASK, new_state);
    }

    /// Reset the register of the specified peripheral to the reset value
    pub fn rst_peripheral(&self, mut peripheral: u32, new_state: bool) {
        if peripheral >= RCC_PERIPHERAL_SYSCFG {
            return;
        }

        if peripheral >= RCC_PERIPHERAL_DMA1 {
            tremo_reg_en!(
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

            tremo_reg_en!(self, rst0, 1 << peripheral, !new_state);
        }
    }

    /// Set the reset mask
    pub fn set_reset_mask(&self, mask: u32) {
        tremo_reg_set!(self, rst_cr, RCC_RST_CR_RESET_REQ_EN_MASK, mask);
    }

    /// Get the reset mask
    pub fn get_reset_mask(&self) -> u32 {
        tremo_reg_rd!(self, rst_cr) & RCC_RST_CR_RESET_REQ_EN_MASK as u32
    }

    /// Set the divider of the I2S MCLK
    pub fn set_i2s_mclk_div(&self, div: u32) {
        tremo_reg_set!(self, cr3, RCC_CR3_I2S_MCLK_DIV_MASK, div << 8);
    }

    /// Set the divider of the I2S SCLK
    pub fn set_i2s_sclk_div(&self, div: u32) {
        tremo_reg_set!(self, cr3, RCC_CR3_I2S_SCLK_DIV_MASK, div);
    }
}
