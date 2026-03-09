use crate::{
    cortex::{VolatileRO, VolatileRW},
    lora::driver::rtc_board::Rtc,
    peripherals::{
        gpio::Gpio, i2c::I2c, i2s::I2s, iwdg::Iwdg, lptimer::Lptimer, lpuart::Lpuart, pwr::Pwr,
        rcc::Rcc, spi::Ssp, timer::TimerGp, uart::Uart, wdg::Wdg,
    },
};

/// Read from analog register at address
#[macro_export]
macro_rules! analog_read {
    ($addr:expr) => {{
        let addr = $addr;
        unsafe {
            core::ptr::read_volatile(
                ($crate::peripherals::regs::AFEC_BASE | (addr << 2)) as *const usize,
            )
        }
    }};
}

/// Write to analog register at address
#[macro_export]
macro_rules! analog_write {
    ($addr:expr, $value:expr) => {
        let addr = $addr;
        let value = $value;
        unsafe {
            core::ptr::write_volatile(
                ($crate::peripherals::regs::AFEC_BASE | (addr << 2)) as *mut usize,
                value,
            )
        }
    };
}

/// Set bits in a register based on a mask and value
#[macro_export]
macro_rules! set_reg_bits {
    ($reg:expr, $mask:expr, $value:expr) => {
        $reg.write(($reg.read() & !($mask as usize)) | ($value as usize));
    };
}

/// Enable or disable bits in a register based on a mask
#[macro_export]
macro_rules! toggle_reg_bits {
    ($reg:expr, $mask:expr, $enable:expr) => {
        $reg.write(if $enable {
            $reg.read() | ($mask as usize)
        } else {
            $reg.read() & !($mask as usize)
        })
    };
}

/// Define a register block and a corresponding wrapper struct for safe access
#[macro_export]
macro_rules! define_reg {
    (
        $(#[$wrapper_meta:meta])*
        $name:ident

        $(#[$raw_meta:meta])*
        $raw_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        #[repr(C)]
        $(#[$raw_meta])*
        pub struct $raw_name {
            $(
                $(#[$field_meta])*
                pub $field: $ty
            ),*
        }

        $(#[$wrapper_meta])*
        pub struct $name {
            ptr: *mut $raw_name,
        }

        unsafe impl ::core::marker::Sync for $name {}

        impl $name {
            pub const fn new(base: usize) -> Self {
                Self {
                    ptr: base as *mut $raw_name,
                }
            }

            #[inline(always)]
            pub const fn ptr(&self) -> *mut $raw_name {
                self.ptr
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = $raw_name;

            fn deref(&self) -> &Self::Target {
                unsafe { &*self.ptr }
            }
        }
    };
}

/// The Flash base address
pub const FLASH_BASE: usize = 0x08000000;
/// The Flash info base address
pub const FLASH_INFO_BASE: usize = 0x10000000;
/// The SYS RAM base address
pub const SRAM_BASE: usize = 0x20000000;
/// The Retention RAM base address
pub const RET_SRAM_BASE: usize = 0x30000000;
/// The peripheral registers base address
pub const PERIPH_BASE: usize = 0x40000000;
/// The AFEC registers base address
pub const AFEC_BASE: usize = 0x40008000;

#[allow(clippy::identity_op)]
pub const RCC_BASE: usize = PERIPH_BASE + 0x00000000;
pub static RCC: Rcc = Rcc::new(RCC_BASE);

pub const RCC_CR0_STCLKEN_SEL_MASK: usize = 0x02000000;
pub const RCC_CR0_STCLKEN_SEL_XO32K: usize = 0x00000000;
pub const RCC_CR0_STCLKEN_SEL_RCO32K: usize = 0x02000000;

pub const RCC_CR0_MCO_CLK_DIV_MASK: usize = 0x01C00000;
pub const RCC_CR0_MCO_CLK_DIV_1: usize = 0x00000000;
pub const RCC_CR0_MCO_CLK_DIV_2: usize = 0x01000000;
pub const RCC_CR0_MCO_CLK_DIV_4: usize = 0x01400000;
pub const RCC_CR0_MCO_CLK_DIV_8: usize = 0x01800000;
pub const RCC_CR0_MCO_CLK_DIV_16: usize = 0x1C00000;

pub const RCC_CR0_MCO_CLK_SEL_MASK: usize = 0x00380000;
pub const RCC_CR0_MCO_CLK_SEL_RCO32K: usize = 0x00000000;
pub const RCC_CR0_MCO_CLK_SEL_XO32K: usize = 0x00080000;
pub const RCC_CR0_MCO_CLK_SEL_RCO4M: usize = 0x00100000;
pub const RCC_CR0_MCO_CLK_SEL_XO24M: usize = 0x00180000;
pub const RCC_CR0_MCO_CLK_SEL_XO32M: usize = 0x00200000;
pub const RCC_CR0_MCO_CLK_SEL_RCO48M: usize = 0x00280000;
pub const RCC_CR0_MCO_CLK_SEL_PLL: usize = 0x00300000;
pub const RCC_CR0_MCO_CLK_SEL_SYSCLCK: usize = 0x00380000;

pub const RCC_CR0_MCO_CLK_OUT_EN_MASK: usize = 0x00040000;

pub const RCC_CR0_PCLK1_DIV_MASK: usize = 0x00038000;
pub const RCC_CR0_PCLK1_DIV_1: usize = 0x00000000;
pub const RCC_CR0_PCLK1_DIV_2: usize = 0x00008000;
pub const RCC_CR0_PCLK1_DIV_4: usize = 0x00010000;
pub const RCC_CR0_PCLK1_DIV_8: usize = 0x00018000;
pub const RCC_CR0_PCLK1_DIV_16: usize = 0x00020000;

pub const RCC_CR0_SYSCLK_SEL_MASK: usize = 0x00007000;
pub const RCC_CR0_SYSCLK_SEL_RCO48M_DIV2: usize = 0x00000000;
pub const RCC_CR0_SYSCLK_SEL_RCO32K: usize = 0x00001000;
pub const RCC_CR0_SYSCLK_SEL_XO32K: usize = 0x00002000;
pub const RCC_CR0_SYSCLK_SEL_PLL: usize = 0x00003000;
pub const RCC_CR0_SYSCLK_SEL_XO24M: usize = 0x00004000;
pub const RCC_CR0_SYSCLK_SEL_XO32M: usize = 0x00005000;
pub const RCC_CR0_SYSCLK_SEL_RCO4M: usize = 0x00006000;
pub const RCC_CR0_SYSCLK_SEL_RCO48M: usize = 0x00007000;

pub const RCC_CR0_HCLK_DIV_MASK: usize = 0x00000F00;
pub const RCC_CR0_HCLK_DIV_1: usize = 0x00000000;
pub const RCC_CR0_HCLK_DIV_2: usize = 0x00000100;
pub const RCC_CR0_HCLK_DIV_4: usize = 0x00000200;
pub const RCC_CR0_HCLK_DIV_8: usize = 0x00000300;
pub const RCC_CR0_HCLK_DIV_16: usize = 0x00000400;
pub const RCC_CR0_HCLK_DIV_32: usize = 0x00000500;
pub const RCC_CR0_HCLK_DIV_64: usize = 0x00000600;
pub const RCC_CR0_HCLK_DIV_128: usize = 0x00000700;
pub const RCC_CR0_HCLK_DIV_256: usize = 0x00000800;
pub const RCC_CR0_HCLK_DIV_512: usize = 0x00000900;

pub const RCC_CR0_PCLK0_DIV_MASK: usize = 0x000000E0;
pub const RCC_CR0_PCLK0_DIV_1: usize = 0x00000000;
pub const RCC_CR0_PCLK0_DIV_2: usize = 0x00000020;
pub const RCC_CR0_PCLK0_DIV_4: usize = 0x00000040;
pub const RCC_CR0_PCLK0_DIV_8: usize = 0x00000060;
pub const RCC_CR0_PCLK0_DIV_16: usize = 0x00000080;

pub const RCC_CR1_LPTIMER1_EXTCLK_SEL_MASK: usize = 0x00000800;

pub const RCC_CR1_LPTIMER0_EXTCLK_SEL_MASK: usize = 0x00000400;

pub const RCC_CR1_LPTIMER1_CLK_SEL_MASK: usize = 0x00000300;
pub const RCC_CR1_LPTIMER1_CLK_SEL_PCLK0: usize = 0x00000000;
pub const RCC_CR1_LPTIMER1_CLK_SEL_RCO4M: usize = 0x00000100;
pub const RCC_CR1_LPTIMER1_CLK_SEL_XO32K: usize = 0x00000200;
pub const RCC_CR1_LPTIMER1_CLK_SEL_RCO32K: usize = 0x00000300;

pub const RCC_CR1_LPTIMER0_CLK_SEL_MASK: usize = 0x000000C0;
pub const RCC_CR1_LPTIMER0_CLK_SEL_PCLK0: usize = 0x00000000;
pub const RCC_CR1_LPTIMER0_CLK_SEL_RCO4M: usize = 0x00000040;
pub const RCC_CR1_LPTIMER0_CLK_SEL_XO32K: usize = 0x00000080;
pub const RCC_CR1_LPTIMER0_CLK_SEL_RCO32K: usize = 0x000000C0;

pub const RCC_CR1_LCD_CLK_SEL_MASK: usize = 0x00000030;
pub const RCC_CR1_LCD_CLK_SEL_XO32K: usize = 0x00000000;
pub const RCC_CR1_LCD_CLK_SEL_RCO32K: usize = 0x00000010;
pub const RCC_CR1_LCD_CLK_SEL_RCO4M: usize = 0x00000020;

pub const RCC_CR1_LPUART_CLK_SEL_MASK: usize = 0x0000000C;
pub const RCC_CR1_LPUART_CLK_SEL_XO32K: usize = 0x00000000;
pub const RCC_CR1_LPUART_CLK_SEL_RCO32K: usize = 0x00000004;
pub const RCC_CR1_LPUART_CLK_SEL_RCO4M: usize = 0x00000008;

pub const RCC_CR1_RTC_CLK_SEL_MASK: usize = 0x00000002;
pub const RCC_CR1_RTC_CLK_SEL_XO32K: usize = 0x00000000;
pub const RCC_CR1_RTC_CLK_SEL_RCO32K: usize = 0x00000002;

pub const RCC_CR1_IWDG_CLK_SEL_MASK: usize = 0x00000001;
pub const RCC_CR1_IWDG_CLK_SEL_XO32K: usize = 0x00000000;
pub const RCC_CR1_IWDG_CLK_SEL_RCO32K: usize = 0x00000001;

pub const RCC_CR2_UART0_CLK_SEL_MASK: usize = 0x00018000;
pub const RCC_CR2_UART0_CLK_SEL_PCLK0: usize = 0x00000000;
pub const RCC_CR2_UART0_CLK_SEL_RCO4M: usize = 0x00008000;
pub const RCC_CR2_UART0_CLK_SEL_XO32K: usize = 0x00010000;
pub const RCC_CR2_UART0_CLK_SEL_XO24M: usize = 0x00018000;

pub const RCC_CR2_UART1_CLK_SEL_MASK: usize = 0x00006000;
pub const RCC_CR2_UART1_CLK_SEL_PCLK0: usize = 0x00000000;
pub const RCC_CR2_UART1_CLK_SEL_RCO4M: usize = 0x00002000;
pub const RCC_CR2_UART1_CLK_SEL_XO32K: usize = 0x00004000;
pub const RCC_CR2_UART1_CLK_SEL_XO24M: usize = 0x00006000;

pub const RCC_CR2_UART2_CLK_SEL_MASK: usize = 0x00001800;
pub const RCC_CR2_UART2_CLK_SEL_PCLK1: usize = 0x00000000;
pub const RCC_CR2_UART2_CLK_SEL_RCO4M: usize = 0x00000800;
pub const RCC_CR2_UART2_CLK_SEL_XO32K: usize = 0x00001000;
pub const RCC_CR2_UART2_CLK_SEL_XO24M: usize = 0x00001800;

pub const RCC_CR2_UART3_CLK_SEL_MASK: usize = 0x00000600;
pub const RCC_CR2_UART3_CLK_SEL_PCLK1: usize = 0x00000000;
pub const RCC_CR2_UART3_CLK_SEL_RCO4M: usize = 0x00000200;
pub const RCC_CR2_UART3_CLK_SEL_XO32K: usize = 0x00000400;
pub const RCC_CR2_UART3_CLK_SEL_XO24M: usize = 0x00000600;

pub const RCC_CR2_SCC_CLK_SEL_MASK: usize = 0x00000180;
pub const RCC_CR2_SCC_CLK_SEL_PCLK1: usize = 0x00000000;
pub const RCC_CR2_SCC_CLK_SEL_SYSCLK: usize = 0x00000080;
pub const RCC_CR2_SCC_CLK_SEL_PLL: usize = 0x00000100;
pub const RCC_CR2_SCC_CLK_SEL_EXT: usize = 0x00000180;

pub const RCC_CR2_ADC_CLK_SEL_MASK: usize = 0x00000060;
pub const RCC_CR2_ADC_CLK_SEL_PCLK1: usize = 0x00000000;
pub const RCC_CR2_ADC_CLK_SEL_SYSCLK: usize = 0x00000020;
pub const RCC_CR2_ADC_CLK_SEL_PLL: usize = 0x00000040;
pub const RCC_CR2_ADC_CLK_SEL_RCO48M: usize = 0x00000060;

pub const RCC_CR2_I2S_CLK_SEL_MASK: usize = 0x0000001C;
pub const RCC_CR2_I2S_CLK_SEL_PCLK0: usize = 0x00000000;
pub const RCC_CR2_I2S_CLK_SEL_XO24M: usize = 0x00000004;
pub const RCC_CR2_I2S_CLK_SEL_PLL: usize = 0x00000008;
pub const RCC_CR2_I2S_CLK_SEL_XO32M: usize = 0x0000000C;
pub const RCC_CR2_I2S_CLK_SEL_EXT_CLK: usize = 0x00000010;

pub const RCC_CR2_QSPI_CLK_SEL_MASK: usize = 0x00000003;
pub const RCC_CR2_QSPI_CLK_SEL_HCLK: usize = 0x00000000;
pub const RCC_CR2_QSPI_CLK_SEL_SYSCLK: usize = 0x00000001;
pub const RCC_CR2_QSPI_CLK_SEL_PLL: usize = 0x00000002;

pub const RCC_CR3_I2S_MCLK_DIV_MASK: usize = 0x0000FF00;

pub const RCC_CR3_I2S_SCLK_DIV_MASK: usize = 0x000000FF;

pub const RCC_CGR0_PWR_CLK_EN_MASK: usize = 0x80000000;
pub const RCC_CGR0_DMAC0_CLK_EN_MASK: usize = 0x40000000;
pub const RCC_CGR0_DMAC1_CLK_EN_MASK: usize = 0x20000000;
pub const RCC_CGR0_CRC_CLK_EN_MASK: usize = 0x10000000;
pub const RCC_CGR0_BSTIMER0_CLK_EN_MASK: usize = 0x08000000;
pub const RCC_CGR0_BSTIMER1_CLK_EN_MASK: usize = 0x04000000;
pub const RCC_CGR0_IOM0_CLK_EN_MASK: usize = 0x02000000;
pub const RCC_CGR0_IOM1_CLK_EN_MASK: usize = 0x01000000;
pub const RCC_CGR0_IOM2_CLK_EN_MASK: usize = 0x00800000;
pub const RCC_CGR0_IOM3_CLK_EN_MASK: usize = 0x00400000;
pub const RCC_CGR0_SYSCFG_CLK_EN_MASK: usize = 0x00200000;
pub const RCC_CGR0_UART0_CLK_EN_MASK: usize = 0x00100000;
pub const RCC_CGR0_UART1_CLK_EN_MASK: usize = 0x00080000;
pub const RCC_CGR0_UART2_CLK_EN_MASK: usize = 0x00040000;
pub const RCC_CGR0_UART3_CLK_EN_MASK: usize = 0x00020000;
pub const RCC_CGR0_LPUART_CLK_EN_MASK: usize = 0x00010000;
pub const RCC_CGR0_SSP0_CLK_EN_MASK: usize = 0x00008000;
pub const RCC_CGR0_SSP1_CLK_EN_MASK: usize = 0x00004000;
pub const RCC_CGR0_SSP2_CLK_EN_MASK: usize = 0x00002000;
pub const RCC_CGR0_I2C0_CLK_EN_MASK: usize = 0x00001000;
pub const RCC_CGR0_I2C1_CLK_EN_MASK: usize = 0x00000800;
pub const RCC_CGR0_I2C2_CLK_EN_MASK: usize = 0x00000400;
pub const RCC_CGR0_SCC_CLK_EN_MASK: usize = 0x00000200;
pub const RCC_CGR0_ADC_CLK_EN_MASK: usize = 0x00000100;
pub const RCC_CGR0_AFEC_CLK_EN_MASK: usize = 0x00000080;
pub const RCC_CGR0_LCD_CLK_EN_MASK: usize = 0x00000040;
pub const RCC_CGR0_DAC_CLK_EN_MASK: usize = 0x00000020;
pub const RCC_CGR0_LORA_CLK_EN_MASK: usize = 0x00000010;
pub const RCC_CGR0_TIMER0_CLK_EN_MASK: usize = 0x00000008;
pub const RCC_CGR0_TIMER1_CLK_EN_MASK: usize = 0x00000004;
pub const RCC_CGR0_TIMER2_CLK_EN_MASK: usize = 0x00000002;
pub const RCC_CGR0_TIMER3_CLK_EN_MASK: usize = 0x00000001;

pub const RCC_CGR1_LPTIMER1_PCLK_EN_MASK: usize = 0x00001000;
pub const RCC_CGR1_LPTIMER1_CLK_EN_MASK: usize = 0x00000800;
pub const RCC_CGR1_RNGC_CLK_EN_MASK: usize = 0x00000400;
pub const RCC_CGR1_LPTIMER0_PCLK_EN_MASK: usize = 0x00000200;
pub const RCC_CGR1_I2S_CLK_EN_MASK: usize = 0x00000100;
pub const RCC_CGR1_SAC_CLK_EN_MASK: usize = 0x00000080;
pub const RCC_CGR1_WDG_CNT_CLK_EN_MASK: usize = 0x00000040;
pub const RCC_CGR1_QSPI_CLK_EN_MASK: usize = 0x00000020;
pub const RCC_CGR1_LPTIMER0_CLK_EN_MASK: usize = 0x00000010;
pub const RCC_CGR1_IWDG_CLK_EN_MASK: usize = 0x00000008;
pub const RCC_CGR1_WDG_CLK_EN_MASK: usize = 0x00000004;
pub const RCC_CGR1_RTC_CLK_EN_MASK: usize = 0x00000002;
pub const RCC_CGR1_SEC_CLK_EN_MASK: usize = 0x00000001;

pub const RCC_CGR2_LPTIMER1_AON_CLK_EN_MASK: usize = 0x00000020;
pub const RCC_CGR2_LPTIMER0_AON_CLK_EN_MASK: usize = 0x00000010;
pub const RCC_CGR2_LCD_AON_CLK_EN_MASK: usize = 0x00000008;
pub const RCC_CGR2_LPUART_AON_CLK_EN_MASK: usize = 0x00000004;
pub const RCC_CGR2_RTC_AON_CLK_EN_MASK: usize = 0x00000002;
pub const RCC_CGR2_IWDG_CLK_EN_MASK: usize = 0x00000001;

pub const RCC_RST0_UART0_RST_N_MASK: usize = 0x80000000;
pub const RCC_RST0_UART1_RST_N_MASK: usize = 0x40000000;
pub const RCC_RST0_UART2_RST_N_MASK: usize = 0x20000000;
pub const RCC_RST0_UART3_RST_N_MASK: usize = 0x10000000;
pub const RCC_RST0_LPUART_RST_N_MASK: usize = 0x08000000;
pub const RCC_RST0_SSP0_RST_N_MASK: usize = 0x04000000;
pub const RCC_RST0_SSP1_RST_N_MASK: usize = 0x02000000;
pub const RCC_RST0_SSP2_RST_N_MASK: usize = 0x01000000;
pub const RCC_RST0_QSPI_RST_N_MASK: usize = 0x00800000;
pub const RCC_RST0_I2C0_RST_N_MASK: usize = 0x00400000;
pub const RCC_RST0_I2C1_RST_N_MASK: usize = 0x00200000;
pub const RCC_RST0_I2C2_RST_N_MASK: usize = 0x00100000;
pub const RCC_RST0_SCC_RST_N_MASK: usize = 0x00080000;
pub const RCC_RST0_ADC_RST_N_MASK: usize = 0x00040000;
pub const RCC_RST0_AFEC_RST_N_MASK: usize = 0x00020000;
pub const RCC_RST0_LCD_RST_N_MASK: usize = 0x00010000;
pub const RCC_RST0_DAC_RST_N_MASK: usize = 0x00008000;
pub const RCC_RST0_LORA_RST_N_MASK: usize = 0x00004000;
pub const RCC_RST0_IOM_RST_N_MASK: usize = 0x00002000;
pub const RCC_RST0_TIMER0_RST_N_MASK: usize = 0x00001000;
pub const RCC_RST0_TIMER1_RST_N_MASK: usize = 0x00000800;
pub const RCC_RST0_TIMER2_RST_N_MASK: usize = 0x00000400;
pub const RCC_RST0_TIMER3_RST_N_MASK: usize = 0x00000200;
pub const RCC_RST0_BSTIMER0_RST_N_MASK: usize = 0x00000100;
pub const RCC_RST0_BSTIMER1_RST_N_MASK: usize = 0x00000080;
pub const RCC_RST0_LPTIMER0_RST_N_MASK: usize = 0x00000040;
pub const RCC_RST0_IWDG_RST_N_MASK: usize = 0x00000020;
pub const RCC_RST0_WDG_RST_N_MASK: usize = 0x00000010;
pub const RCC_RST0_RTC_RST_N_MASK: usize = 0x00000008;
pub const RCC_RST0_CRC_RST_N_MASK: usize = 0x00000004;
pub const RCC_RST0_SEC_RST_N_MASK: usize = 0x00000002;
pub const RCC_RST0_SAC_RST_N_MASK: usize = 0x00000001;

pub const RCC_RST1_LPTIMER1_RST_N_MASK: usize = 0x00000010;
pub const RCC_RST1_RNGC_RST_N_MASK: usize = 0x00000008;
pub const RCC_RST1_I2S_RST_N_MASK: usize = 0x00000004;
pub const RCC_RST1_DMAC0_RST_N_MASK: usize = 0x00000002;
pub const RCC_RST1_DMAC1_RST_N_MASK: usize = 0x00000001;

pub const RCC_RST_SR_BOR_RESET_SR: usize = 0x00000040;
pub const RCC_RST_SR_IWDG_RESET_SR: usize = 0x00000020;
pub const RCC_RST_SR_WDG_RESET_SR: usize = 0x00000010;
pub const RCC_RST_SR_EFC_RESET_SR: usize = 0x00000008;
pub const RCC_RST_SR_CPU_RESET_SR: usize = 0x00000004;
pub const RCC_RST_SR_SEC_RESET_SR: usize = 0x00000002;
pub const RCC_RST_SR_STANDBY_RESET_SR: usize = 0x00000001;

pub const RCC_RST_CR_RESET_REQ_EN_MASK: usize = 0x0000003E;
pub const RCC_RST_CR_IWDG_RESET_REQ_EN_MASK: usize = 0x00000020;
pub const RCC_RST_CR_WDG_RESET_REQ_EN_MASK: usize = 0x00000010;
pub const RCC_RST_CR_EFC_RESET_REQ_EN_MASK: usize = 0x00000008;
pub const RCC_RST_CR_CPU_RESET_REQ_EN_MASK: usize = 0x00000004;
pub const RCC_RST_CR_SEC_RESET_REQ_EN_MASK: usize = 0x00000002;

pub const RCC_SR_ALL_DONE: usize = 0x0000003F;
pub const RCC_SR_LPTIMER1_AON_CLK_EN_DONE: usize = 0x00000020;
pub const RCC_SR_LPTIM_AON_CLK_EN_DONE: usize = 0x00000010;
pub const RCC_SR_LCD_AON_CLK_EN_DONE: usize = 0x00000008;
pub const RCC_SR_LPUART_AON_CLK_EN_DONE: usize = 0x00000004;
pub const RCC_SR_RTC_AON_CLK_EN_DONE: usize = 0x00000002;
pub const RCC_SR_IWDG_AON_CLK_EN_DONE: usize = 0x00000001;

pub const RCC_SR1_LPTIMER1_CLK_EN_SYNC: usize = 0x00100000;
pub const RCC_SR1_LPTIMER1_AON_CLK_EN_SYNC: usize = 0x00080000;
pub const RCC_SR1_UART0_CLK_EN_SYNC: usize = 0x00040000;
pub const RCC_SR1_UART1_CLK_EN_SYNC: usize = 0x00020000;
pub const RCC_SR1_UART2_CLK_EN_SYNC: usize = 0x00010000;
pub const RCC_SR1_UART3_CLK_EN_SYNC: usize = 0x00008000;
pub const RCC_SR1_SCC_CLK_EN_SYNC: usize = 0x00004000;
pub const RCC_SR1_ADC_CLK_EN_SYNC: usize = 0x00002000;
pub const RCC_SR1_LPTIMER0_CLK_EN_SYNC: usize = 0x00001000;
pub const RCC_SR1_QSPI_CLK_EN_SYNC: usize = 0x00000800;
pub const RCC_SR1_LPUART_CLK_EN_SYNC: usize = 0x00000400;
pub const RCC_SR1_LCD_CLK_EN_SYNC: usize = 0x00000200;
pub const RCC_SR1_IWDG_CLK_EN_SYNC: usize = 0x00000100;
pub const RCC_SR1_RTC_CLK_EN_SYNC: usize = 0x00000080;
pub const RCC_SR1_MCO_CLK_EN_SYNC: usize = 0x00000040;
pub const RCC_SR1_I2S_CLK_EN_SYNC: usize = 0x00000020;
pub const RCC_SR1_LPTIMER0_AON_CLK_EN_SYNC: usize = 0x00000010;
pub const RCC_SR1_LCD_AON_CLK_EN_SYNC: usize = 0x00000008;
pub const RCC_SR1_LPUART_AON_CLK_EN_SYNC: usize = 0x00000004;
pub const RCC_SR1_RTC_AON_CLK_EN_SYNC: usize = 0x00000002;
pub const RCC_SR1_IWDG_AON_CLK_EN_SYNC: usize = 0x00000001;

pub const SSP0_BASE: usize = PERIPH_BASE + 0x00006000;
pub const SSP1_BASE: usize = PERIPH_BASE + 0x00012000;
pub const SSP2_BASE: usize = PERIPH_BASE + 0x00013000;
pub static SSP0: Ssp = Ssp::new(SSP0_BASE);
pub static SSP1: Ssp = Ssp::new(SSP1_BASE);
pub static SSP2: Ssp = Ssp::new(SSP2_BASE);

pub const SSP_NUM_PORTS: usize = 3;

pub const GPIO_BASE: usize = 0x4001F000;
pub const GPIOA_BASE: usize = GPIO_BASE;
pub const GPIOB_BASE: usize = GPIO_BASE + 0x400;
pub const GPIOC_BASE: usize = GPIO_BASE + 0x800;
pub const GPIOD_BASE: usize = GPIO_BASE + 0xC00;
pub static GPIOA: Gpio = Gpio::new(GPIOA_BASE);
pub static GPIOB: Gpio = Gpio::new(GPIOB_BASE);
pub static GPIOC: Gpio = Gpio::new(GPIOC_BASE);
pub static GPIOD: Gpio = Gpio::new(GPIOD_BASE);

pub const RTC_REG_BASE: usize = 0x4000E000;
pub static RTC: Rtc = Rtc::new(RTC_REG_BASE);

pub const UART0_BASE: usize = PERIPH_BASE + 0x3000;
pub const UART1_BASE: usize = PERIPH_BASE + 0x4000;
pub const UART2_BASE: usize = PERIPH_BASE + 0x10000;
pub const UART3_BASE: usize = PERIPH_BASE + 0x11000;

pub static UART0: Uart = Uart::new(UART0_BASE);
pub static UART1: Uart = Uart::new(UART1_BASE);
pub static UART2: Uart = Uart::new(UART2_BASE);
pub static UART3: Uart = Uart::new(UART3_BASE);

/****************************UART CR bit definition***************************/
pub const UART_CR_SIR_LPIRDA_EN: usize = 0x00000004;
pub const UART_CR_SIR_EN: usize = 0x00000002;

pub const UART_CR_UART_EN: usize = 0x00000001;

pub const UART_CR_UART_MODE: usize = 0x00000030;
pub const UART_CR_UART_MODE_NONE: usize = 0x00000000;
pub const UART_CR_UART_MODE_RX: usize = 0x00000020;
pub const UART_CR_UART_MODE_TX: usize = 0x00000010;
pub const UART_CR_UART_MODE_TXRX: usize = 0x00000030;

pub const UART_CR_FLOW_CTRL: usize = 0x0000C000;
pub const UART_CR_FLOW_CTRL_NONE: usize = 0x00000000;
pub const UART_CR_FLOW_CTRL_CTS: usize = 0x00008000;
pub const UART_CR_FLOW_CTRL_RTS: usize = 0x00004000;
pub const UART_CR_FLOW_CTRL_CTS_RTS: usize = 0x0000C000;

/****************************UART LCR_H bit definition*************************/
pub const UART_LCR_H_PEN: usize = 0x00000002;

pub const UART_LCR_H_EPS_EVEN: usize = 0x00000004;

pub const UART_LCR_H_STOP: usize = 0x00000008;
pub const UART_LCR_H_STOP_1: usize = 0x00000000;
pub const UART_LCR_H_STOP_2: usize = 0x00000008;

pub const UART_LCR_H_FEN: usize = 0x00000010;

pub const UART_LCR_H_WLEN: usize = 0x00000060;
pub const UART_LCR_H_WLEN_5: usize = 0x00000000;
pub const UART_LCR_H_WLEN_6: usize = 0x00000020;
pub const UART_LCR_H_WLEN_7: usize = 0x00000040;
pub const UART_LCR_H_WLEN_8: usize = 0x00000060;

/****************************UART IFLS bit definition**************************/
pub const UART_IFLS_TX: usize = 0x00000007;
pub const UART_IFLS_TX_1_8: usize = 0x00000000;
pub const UART_IFLS_TX_1_4: usize = 0x00000001;
pub const UART_IFLS_TX_1_2: usize = 0x00000002;
pub const UART_IFLS_TX_3_4: usize = 0x00000003;
pub const UART_IFLS_TX_7_8: usize = 0x00000004;

pub const UART_IFLS_RX: usize = 0x00000038;
pub const UART_IFLS_RX_1_8: usize = 0x00000000;
pub const UART_IFLS_RX_1_4: usize = 0x00000008;
pub const UART_IFLS_RX_1_2: usize = 0x00000010;
pub const UART_IFLS_RX_3_4: usize = 0x00000018;
pub const UART_IFLS_RX_7_8: usize = 0x00000020;

/****************************UART DMACR bit definition*************************/
pub const UART_DMACR_ONERR_EN_MASK: usize = 0x00000004;

pub const UART_DMACR_TX_EN_MASK: usize = 0x00000002;

pub const UART_DMACR_RX_EN_MASK: usize = 0x00000001;

pub const LPUART_BASE: usize = PERIPH_BASE + 0x5000;
pub static LPUART: Lpuart = Lpuart::new(LPUART_BASE);

define_reg! {
    Efc
    __Efc {
        cr: VolatileRW<usize>,
        int_en: VolatileRW<usize>,
        sr: VolatileRW<usize>,
        program_data0: VolatileRW<usize>,
        program_data1: VolatileRW<usize>,
        timing_cfg: VolatileRW<usize>,
        protect_seq: VolatileRW<usize>,
        rsv0: VolatileRW<usize>,
        chip_pattern: VolatileRO<usize>,
        ip_trim_l: VolatileRO<usize>,
        ip_trim_h: VolatileRO<usize>,
        sn_l: VolatileRO<usize>,
        sn_h: VolatileRO<usize>,
        test_info_l: VolatileRO<usize>,
        test_info_h: VolatileRO<usize>,
        option_csr_bytes: VolatileRO<usize>,
        option_e0_bytes: VolatileRO<usize>,
        option_wp_bytes: VolatileRO<usize>,
        option_sec_bytes0: VolatileRO<usize>,
        option_sec_bytes1: VolatileRO<usize>,
    }
}

pub const EFC_BASE: usize = PERIPH_BASE + 0x20000;
pub static EFC: Efc = Efc::new(EFC_BASE);

/****************************EFC CR bit definition*****************************/
pub const EFC_CR_INFO_LOAD_MASK: usize = 0x80000000;
pub const EFC_CR_ECC_DISABLE_MASK: usize = 0x00000200;
pub const EFC_CR_OPTION_OP_EN_MASK: usize = 0x00000100;
pub const EFC_CR_FACTORY_OP_EN_MASK: usize = 0x00000080;
pub const EFC_CR_WRITE_RELEASE_EN_MASK: usize = 0x00000040;
pub const EFC_CR_PREFETCH_EN_MASK: usize = 0x00000020;
pub const EFC_CR_READ_ACC_EN_MASK: usize = 0x00000010;

pub const EFC_CR_PROG_MODE_MASK: usize = 0x00000008;
pub const EFC_CR_PROG_MODE_DWORD: usize = 0x00000000;
pub const EFC_CR_PROG_MODE_WLINE: usize = 0x00000008;

pub const EFC_CR_PROG_EN_MASK: usize = 0x00000004;
pub const EFC_CR_PAGE_ERASE_EN_MASK: usize = 0x00000002;
pub const EFC_CR_MASS_ERASE_EN_MASK: usize = 0x00000001;

/****************************EFC INT_EN bit definition*************************/

/****************************EFC TIMING_CFG bit definition*********************/
pub const EFC_TIMING_CFG_READ_NUM_MASK: usize = 0x000F0000;

/****************************EFC SR bit definition*****************************/
pub const EFC_SR_OPTION_WRITE_ERROR: usize = 0x00000010;
pub const EFC_SR_PROGRAM_DATA_WAIT: usize = 0x00000004;
pub const EFC_SR_READ_NUM_DONE: usize = 0x00000002;
pub const EFC_SR_OPERATION_DONE: usize = 0x00000001;

define_reg! {
    Lorac
    __Lorac {
        ssp_cr0: VolatileRW<usize>,
        ssp_cr1: VolatileRW<usize>,
        ssp_dr: VolatileRW<usize>,
        ssp_sr: VolatileRO<usize>,
        ssp_cpsr: VolatileRW<usize>,
        ssp_imsc: VolatileRW<usize>,
        ssp_ris: VolatileRO<usize>,
        ssp_mis: VolatileRO<usize>,
        ssp_icr: VolatileRW<usize>,
        ssp_dma_cr: VolatileRW<usize>,
        rsv: [VolatileRO<usize>; 54],
        cr0: VolatileRW<usize>,
        cr1: VolatileRW<usize>,
        sr: VolatileRO<usize>,
        nss_cr: VolatileRW<usize>,
        sck_cr: VolatileRW<usize>,
        mosi_cr: VolatileRW<usize>,
        miso_sr: VolatileRW<usize>,
    }
}

pub const LORAC_BASE: usize = PERIPH_BASE + 0x9000;
pub static LORAC: Lorac = Lorac::new(LORAC_BASE);

define_reg! {
    Afec
    __Afec {
        cr: VolatileRW<usize>,
        int_sr: VolatileRW<usize>,
        raw_sr: VolatileRO<usize>,
    }
}
pub static AFEC: Afec = Afec::new(AFEC_BASE + 0x200);

pub const AFEC_RAW_SR_RCO4M_READY_MASK: usize = 0x80000000;
pub const AFEC_RAW_SR_PLL_UNLOCK_MASK: usize = 0x40000000;
pub const AFEC_RAW_SR_RCO24M_READY_MASK: usize = 0x00000004;

pub const IWDG_BASE: usize = PERIPH_BASE + 0x1D000;
pub static IWDG: Iwdg = Iwdg::new(IWDG_BASE);

/****************************IWDG CR bit definition*************************/
pub const IWDG_CR_RSTEN_MASK: usize = 0x00000020;
pub const IWDG_CR_WKEN_MASK: usize = 0x00000010;

pub const IWDG_CR_PREDIV_MASK: usize = 0x0000000E;
pub const IWDG_CR_PREDIV_4: usize = 0x00000000;
pub const IWDG_CR_PREDIV_8: usize = 0x00000002;
pub const IWDG_CR_PREDIV_16: usize = 0x00000004;
pub const IWDG_CR_PREDIV_32: usize = 0x00000006;
pub const IWDG_CR_PREDIV_64: usize = 0x00000008;
pub const IWDG_CR_PREDIV_128: usize = 0x0000000A;
pub const IWDG_CR_PREDIV_256: usize = 0x0000000C;

pub const IWDG_CR_START_MASK: usize = 0x00000001;

/****************************IWDG SR bit definition*************************/
pub const IWDG_SR_WRITE_SR2_DONE: usize = 0x00000008;
pub const IWDG_SR_WIN_SET_DONE: usize = 0x00000004;
pub const IWDG_SR_MAX_SET_DONE: usize = 0x00000002;
pub const IWDG_SR_WRITE_CR_DONE: usize = 0x00000001;

/****************************IWDG SR1 bit definition************************/
pub const IWDG_SR1_RESET_REQ_SYNC: usize = 0x00001000;

/****************************IWDG CR1 bit definition************************/
pub const IWDG_CR1_RESET_REQ_RST_EN_MASK: usize = 0x00000002;
pub const IWDG_CR1_RESET_REQ_INT_EN_MASK: usize = 0x00000001;

/****************************IWDG SR2 bit definition************************/
pub const IWDG_SR2_RESET_REQ_SR_MASK: usize = 0x00000001;

pub const WDG_BASE: usize = PERIPH_BASE + 0x1E000;
pub static WDG: Wdg = Wdg::new(WDG_BASE);

define_reg! {
    Crc
    __Crc {
        cr: VolatileRW<usize>,
        dr: VolatileRW<usize>,
        init: VolatileRW<usize>,
        poly: VolatileRW<usize>,
    }
}

/// CRC base address
pub const CRC_BASE: usize = PERIPH_BASE + 0x22000;
/// CRC peripheral
pub static CRC: Crc = Crc::new(CRC_BASE);

/****************************CRC CR bit definition*****************************/
pub const CRC_CR_CALC_FLAG: usize = 0x00000040;

pub const CRC_CR_CALC_INIT: usize = 0x00000020;

pub const CRC_CR_POLY_SIZE_MASK: usize = 0x00000018;
pub const CRC_CR_POLY_SIZE_7: usize = 0x00000018;
pub const CRC_CR_POLY_SIZE_8: usize = 0x00000010;
pub const CRC_CR_POLY_SIZE_16: usize = 0x00000008;
pub const CRC_CR_POLY_SIZE_32: usize = 0x00000000;

pub const CRC_CR_REVERSE_IN_MASK: usize = 0x00000006;
pub const CRC_CR_REVERSE_IN_NONE: usize = 0x00000000;
pub const CRC_CR_REVERSE_IN_BYTE: usize = 0x00000002;
pub const CRC_CR_REVERSE_IN_HWORD: usize = 0x00000004;
pub const CRC_CR_REVERSE_IN_WORD: usize = 0x00000006;

pub const CRC_CR_REVERSE_OUT_EN: usize = 0x00000001;

pub const I2C0_BASE: usize = PERIPH_BASE + 0x7000;
pub const I2C1_BASE: usize = PERIPH_BASE + 0x14000;
pub const I2C2_BASE: usize = PERIPH_BASE + 0x15000;

pub static I2C0: I2c = I2c::new(I2C0_BASE);
pub static I2C1: I2c = I2c::new(I2C1_BASE);
pub static I2C2: I2c = I2c::new(I2C2_BASE);

pub const I2C_CR_RFIFO_OVERRUN_INTR_EN_MASK: usize = 0x80000000;
pub const I2C_CR_RFIFO_FULL_INTR_EN_MASK: usize = 0x40000000;
pub const I2C_CR_RFIFO_HALFFULL_INTR_EN_MASK: usize = 0x20000000;
pub const I2C_CR_TFIFO_EMPTY_INTR_EN_MASK: usize = 0x10000000;
pub const I2C_CR_TRANS_DONE_INTR_EN_MASK: usize = 0x08000000;
pub const I2C_CR_MASTER_STOP_DET_EN_MASK: usize = 0x04000000;
pub const I2C_CR_MASTER_STOP_DET_INTR_EN_MASK: usize = 0x02000000;
pub const I2C_CR_SLAVE_STOP_DET_INTR_EN_MASK: usize = 0x01000000;
pub const I2C_CR_SLAVE_ADDR_DET_INTR_EN_MASK: usize = 0x00800000;
pub const I2C_CR_BUS_ERROR_INTR_EN_MASK: usize = 0x00400000;
pub const I2C_CR_GENERAL_CALL_DIS_MASK: usize = 0x00200000;
pub const I2C_CR_DBR_FULL_INTR_EN_MASK: usize = 0x00100000;
pub const I2C_CR_IDBR_EMPTY_INTR_EN_MASK: usize = 0x00080000;
pub const I2C_CR_ARB_LOSS_DET_INTR_EN_MASK: usize = 0x00040000;

pub const I2C_CR_TWSI_UNIT_EN_MASK: usize = 0x00004000;
pub const I2C_CR_SCL_EN_MASK: usize = 0x00002000;
pub const I2C_CR_MASTER_ABORT_MASK: usize = 0x00001000;
pub const I2C_CR_BUS_RESET_REQUEST_MASK: usize = 0x00000800;
pub const I2C_CR_UNIT_RESET_MASK: usize = 0x00000400;

pub const I2C_CR_BUS_MODE_MASK: usize = 0x00000300;
pub const I2C_CR_BUS_MODE_STANDARD: usize = 0x00000000;
pub const I2C_CR_BUS_MODE_FAST: usize = 0x00000100;
pub const I2C_CR_BUS_MODE_HIGH: usize = 0x00000200;

pub const I2C_CR_DMA_EN_MASK: usize = 0x00000080;

pub const I2C_CR_FIFO_EN_MASK: usize = 0x00000020;

pub const I2C_CR_TRANS_BEGIN_MASK: usize = 0x00000010;
pub const I2C_CR_TRANS_BYTE_MASK: usize = 0x00000008;
pub const I2C_CR_ACKNAK_MASK: usize = 0x00000004;
pub const I2C_CR_STOP_MASK: usize = 0x00000002;
pub const I2C_CR_START_MASK: usize = 0x00000001;

/****************************I2C SR bit definition*****************************/
pub const I2C_SR_RFIFO_OVERRUN_MASK: usize = 0x80000000;
pub const I2C_SR_RFIFO_FULL_MASK: usize = 0x40000000;
pub const I2C_SR_RFIFO_HALFFULL_MASK: usize = 0x20000000;
pub const I2C_SR_TFIFO_EMPTY_MASK: usize = 0x10000000;
pub const I2C_SR_TRANS_DONE_MASK: usize = 0x08000000;
pub const I2C_SR_MASTER_STOP_DET_MASK: usize = 0x02000000;
pub const I2C_SR_SLAVE_STOP_DET_MASK: usize = 0x01000000;
pub const I2C_SR_SLAVE_ADDR_DET_MASK: usize = 0x00800000;
pub const I2C_SR_BUS_ERROR_MASK: usize = 0x00400000;
pub const I2C_SR_GENERAL_CALL_MASK: usize = 0x00200000;
pub const I2C_SR_DBR_FULL_MASK: usize = 0x00100000;
pub const I2C_SR_IDBR_EMPTY_MASK: usize = 0x00080000;
pub const I2C_SR_ARB_LOSS_DET_MASK: usize = 0x00040000;

pub const I2C_SR_BUS_BUSY_MASK: usize = 0x00010000;
pub const I2C_SR_UNIT_BUSY_MASK: usize = 0x00008000;
pub const I2C_SR_ACK_STATUS_MASK: usize = 0x00004000;
pub const I2C_SR_RW_MODE_MASK: usize = 0x00002000;

/****************************I2C WFIFO bit definition************************/
pub const I2C_WFIFO_CONTROL_TB_MASK: usize = 0x00000800;
pub const I2C_WFIFO_CONTROL_ACKNAK_MASK: usize = 0x00000400;
pub const I2C_WFIFO_CONTROL_STOP_MASK: usize = 0x00000200;
pub const I2C_WFIFO_CONTROL_START_MASK: usize = 0x00000100;

/****************************I2C WFIFO_STATUS bit definition*****************/
pub const I2C_WFIFO_STATUS_SIZE_MASK: usize = 0x0000003C;
pub const I2C_WFIFO_STATUS_EMPTY_MASK: usize = 0x00000002;
pub const I2C_WFIFO_STATUS_FULL_MASK: usize = 0x00000001;

/****************************I2C RFIFO_STATUS bit definition*****************/
pub const I2C_RFIFO_STATUS_SIZE_MASK: usize = 0x000000F0;
pub const I2C_RFIFO_STATUS_EMPTY_MASK: usize = 0x00000004;
pub const I2C_RFIFO_STATUS_FULL_MASK: usize = 0x00000003;
pub const I2C_RFIFO_STATUS_HFULL_MASK: usize = 0x00000002;
pub const I2C_RFIFO_STATUS_OVERUN_MASK: usize = 0x00000001;

define_reg! {
    Syscfg
    __Syscfg {
        cr0: VolatileRW<usize>,
        cr1: VolatileRW<usize>,
        cr2: VolatileRW<usize>,
        cr3: VolatileRW<usize>,
        cr4: VolatileRW<usize>,
        cr5: VolatileRW<usize>,
        cr6: VolatileRW<usize>,
        cr7: VolatileRW<usize>,
        cr8: VolatileRW<usize>,
        cr9: VolatileRW<usize>,
        cr10: VolatileRW<usize>,
    }
}

pub const SYSCFG_BASE: usize = PERIPH_BASE + 0x1000;
pub static SYSCFG: Syscfg = Syscfg::new(SYSCFG_BASE);

pub const PWR_BASE: usize = PERIPH_BASE + 0x1800;
pub static PWR: Pwr = Pwr::new(PWR_BASE);

pub const TIMER0_SFR_BASE: usize = 0x4000A000;
pub const TIMER1_SFR_BASE: usize = 0x4001A000;
pub const TIMER2_SFR_BASE: usize = 0x4000B000;
pub const TIMER3_SFR_BASE: usize = 0x4001B000;

pub static TIMER0: TimerGp = TimerGp::new(TIMER0_SFR_BASE);
pub static TIMER1: TimerGp = TimerGp::new(TIMER1_SFR_BASE);
pub static TIMER2: TimerGp = TimerGp::new(TIMER2_SFR_BASE);
pub static TIMER3: TimerGp = TimerGp::new(TIMER3_SFR_BASE);

pub const LPTIMER0_SFR_BASE: usize = 0x4000D000;
pub const LPTIMER1_SFR_BASE: usize = 0x4000D800;

pub static LPTIMER0: Lptimer = Lptimer::new(LPTIMER0_SFR_BASE);
pub static LPTIMER1: Lptimer = Lptimer::new(LPTIMER1_SFR_BASE);

pub const I2S_BASE: usize = 0x40002000;
pub static I2S: I2s = I2s::new(I2S_BASE);

define_reg! {
    Bstimer
    __Bstimer {
        cr1: VolatileRW<usize>,
        cr2: VolatileRW<usize>,
        resv1: VolatileRO<usize>,
        dier: VolatileRW<usize>,
        sr: VolatileRO<usize>,
        egr: VolatileRW<usize>,
        resv2: [VolatileRO<usize>; 3],
        cnt: VolatileRW<usize>,
        psc: VolatileRW<usize>,
        arr: VolatileRW<usize>,
    }
}

pub const BSTIMER0_SFR_BASE: usize = 0x4000C000;
pub const BSTIMER1_SFR_BASE: usize = 0x4001C000;
pub static BSTIMER0: Bstimer = Bstimer::new(BSTIMER0_SFR_BASE);
pub static BSTIMER1: Bstimer = Bstimer::new(BSTIMER1_SFR_BASE);

define_reg! {
    Sec
    __Sec {
        int: VolatileRW<usize>,
        rst: VolatileRW<usize>,
        sr: VolatileRW<usize>,
        filter0: VolatileRW<usize>,
        filter1: VolatileRW<usize>,
        filter2: VolatileRW<usize>,
        filter3: VolatileRW<usize>,
    }
}

pub const SEC_BASE: usize = 0x4000F000;
pub static SEC: Sec = Sec::new(SEC_BASE);

pub const SEC_SR_FLASH_ACCESS_ERROR_MASK: usize = 0x00001000;

define_reg! {
    Qspi
    __Qspi {
        qspi_cr: VolatileRW<usize>,
        qspi_dcr: VolatileRW<usize>,
        qspi_sr: VolatileRW<usize>,
        qspi_fcr: VolatileRW<usize>,
        qspi_dlr: VolatileRW<usize>,
        qspi_ccr: VolatileRW<usize>,
        qspi_ar: VolatileRW<usize>,
        qspi_abr: VolatileRW<usize>,
        qspi_dr: VolatileRW<usize>,
        qspi_psmkr: VolatileRW<usize>,
        qspi_psmar: VolatileRW<usize>,
        qspi_pir: VolatileRW<usize>,
        qspi_tor: VolatileRW<usize>,
        reserved: [VolatileRW<usize>; 19],
        qspi_hit0r: VolatileRW<usize>,
        qspi_hit1r: VolatileRW<usize>,
        qspi_mir: VolatileRW<usize>,
        qspi_cfgr: VolatileRW<usize>,
        sbus_start: VolatileRW<usize>,
    }
}

pub const QSPI_BASE: usize = 0x40021000;
pub static QSPI: Qspi = Qspi::new(QSPI_BASE);

define_reg! {
    Dac
    __Dac {
        cr: VolatileRW<usize>,
        swtrigr: VolatileRW<usize>,
        dhr: VolatileRW<usize>,
        dor: VolatileRO<usize>,
        sr: VolatileRW<usize>,
    }
}

pub const DAC_BASE: usize = 0x40019000;
pub static DAC: Dac = Dac::new(DAC_BASE);

pub const DAC_CR_INTR_EMPTY_EN_MASK: usize = 0x00010000;
pub const DAC_CR_INTR_UNDERFLOW_EN_MASK: usize = 0x00008000;

pub const DAC_CR_DMA_EN_MASK: usize = 0x00004000;

pub const DAC_CR_MASK_AMP_SEL_MASK: usize = 0x00003C00;
pub const DAC_CR_MASK_AMP_SEL_1: usize = 0x00000000;
pub const DAC_CR_MASK_AMP_SEL_3: usize = 0x00000400;
pub const DAC_CR_MASK_AMP_SEL_7: usize = 0x00000800;
pub const DAC_CR_MASK_AMP_SEL_15: usize = 0x00000C00;
pub const DAC_CR_MASK_AMP_SEL_31: usize = 0x00001000;
pub const DAC_CR_MASK_AMP_SEL_63: usize = 0x00001400;
pub const DAC_CR_MASK_AMP_SEL_127: usize = 0x00001800;
pub const DAC_CR_MASK_AMP_SEL_255: usize = 0x00001C00;
pub const DAC_CR_MASK_AMP_SEL_511: usize = 0x00002000;
pub const DAC_CR_MASK_AMP_SEL_1023: usize = 0x00002400;

pub const DAC_CR_WAVE_SEL_MASK: usize = 0x00000300;
pub const DAC_CR_WAVE_SEL_NONE: usize = 0x00000000;
pub const DAC_CR_WAVE_SEL_NOISE: usize = 0x00000100;
pub const DAC_CR_WAVE_SEL_TRIANGLE: usize = 0x00000200;

pub const DAC_CR_TRIG_TYPE_SEL_MASK: usize = 0x000000C0;
pub const DAC_CR_TRIG_TYPE_SEL_RISING_EDGE: usize = 0x00000000;
pub const DAC_CR_TRIG_TYPE_SEL_FALLING_EDGE: usize = 0x00000040;
pub const DAC_CR_TRIG_TYPE_SEL_RISING_FALLING_EDGE: usize = 0x00000080;

pub const DAC_CR_TRIG_SRC_SEL_MASK: usize = 0x00000038;
pub const DAC_CR_TRIG_SRC_SEL_GPTIMER1_TRGO: usize = 0x00000000;
pub const DAC_CR_TRIG_SRC_SEL_GPTIMER0_TRGO: usize = 0x00000008;
pub const DAC_CR_TRIG_SRC_SEL_BSTIMER1_TRGO: usize = 0x00000010;
pub const DAC_CR_TRIG_SRC_SEL_BSTIMER0_TRGO: usize = 0x00000018;
pub const DAC_CR_TRIG_SRC_SEL_GPIO6: usize = 0x00000020;
pub const DAC_CR_TRIG_SRC_SEL_GPIO24: usize = 0x00000028;
pub const DAC_CR_TRIG_SRC_SEL_GPIO43: usize = 0x00000030;
pub const DAC_CR_TRIG_SRC_SEL_SOFTWARE: usize = 0x00000038;

pub const DAC_CR_TRIG_EN_MASK: usize = 0x00000004;

pub const DAC_CR_DAC_EN_MASK: usize = 0x00000001;

define_reg! {
    Adc
    __Adc {
        cr: VolatileRW<usize>,
        cfgr: VolatileRW<usize>,
        seqr0: VolatileRW<usize>,
        seqr1: VolatileRW<usize>,
        diffsel: VolatileRW<usize>,
        isr: VolatileRW<usize>,
        ier: VolatileRW<usize>,
        dr: VolatileRO<usize>,
        awd0_cfgr: VolatileRW<usize>,
        awd1_cfgr: VolatileRW<usize>,
        awd2_cfgr: VolatileRW<usize>,
    }
}

pub const ADC_BASE: usize = 0x40017000;
pub static ADC: Adc = Adc::new(ADC_BASE);

define_reg! {
    Lcd
    __Lcd {
        cr0: VolatileRW<usize>,
        cr1: VolatileRW<usize>,
        dr0: VolatileRW<usize>,
        dr1: VolatileRW<usize>,
        dr2: VolatileRW<usize>,
        dr3: VolatileRW<usize>,
        dr4: VolatileRW<usize>,
        dr5: VolatileRW<usize>,
        dr6: VolatileRW<usize>,
        dr7: VolatileRW<usize>,
        sr: VolatileRO<usize>,
        cr2: VolatileRW<usize>,
    }
}

pub const LCD_BASE: usize = 0x40018000;
pub static LCD: Lcd = Lcd::new(LCD_BASE);
