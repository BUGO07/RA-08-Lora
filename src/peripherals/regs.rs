#![allow(clippy::identity_op)]

/// Uart Status
#[repr(u32)]
pub enum SetStatus {
    Reset = 0,
    Set = !0,
}

/// Read from analog register at address
#[macro_export]
macro_rules! analog_read {
    ($addr:expr) => {{
        let addr = $addr;
        unsafe {
            core::ptr::read_volatile(
                ($crate::peripherals::regs::AFEC_BASE | (addr << 2)) as *const u32,
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
                ($crate::peripherals::regs::AFEC_BASE | (addr << 2)) as *mut u32,
                value,
            )
        }
    };
}

/// Set bits in a register based on a mask and value
#[macro_export]
macro_rules! set_reg_bits {
    ($obj:expr, $reg:ident, $mask:expr, $value:expr) => {
        $obj.$reg = ($obj.$reg & !($mask as u32)) | ($value as u32);
    };
}

/// Enable or disable bits in a register based on a mask
#[macro_export]
macro_rules! toggle_reg_bits {
    ($obj:expr, $reg:ident, $mask:expr, $enable:expr) => {
        $obj.$reg = if $enable {
            $obj.$reg | ($mask as u32)
        } else {
            $obj.$reg & !($mask as u32)
        };
    };
}

/// Define a register block and a corresponding wrapper struct for safe access
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

        #[derive(Clone)]
        $(#[$wrapper_meta])*
        pub struct $name {
            ptr: *mut $raw_name,
        }

        unsafe impl Sync for $name {}

        impl $name {
            pub const fn new(base: u32) -> Self {
                Self {
                    ptr: base as *mut $raw_name,
                }
            }

            #[inline(always)]
            pub const fn ptr(&self) -> *mut $raw_name {
                self.ptr
            }
        }

        impl core::ops::Deref for $name {
            type Target = $raw_name;

            fn deref(&self) -> &Self::Target {
                unsafe { &*self.ptr }
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *self.ptr }
            }
        }
    };
}

/// The Flash base address
pub const FLASH_BASE: u32 = 0x08000000;
/// The Flash info base address
pub const FLASH_INFO_BASE: u32 = 0x10000000;
/// The SYS RAM base address
pub const SRAM_BASE: u32 = 0x20000000;
/// The Retention RAM base address
pub const RET_SRAM_BASE: u32 = 0x30000000;
/// The peripheral registers base address
pub const PERIPH_BASE: u32 = 0x40000000;
/// The AFEC registers base address
pub const AFEC_BASE: u32 = 0x40008000;

define_reg! {
    Rcc
    __Rcc {
        cr0: u32,
        cr1: u32,
        cr2: u32,
        cgr0: u32,
        cgr1: u32,
        cgr2: u32,
        rst0: u32,
        rst1: u32,
        rst_sr: u32,
        rst_cr: u32,
        sr: u32,
        sr1: u32,
        cr3: u32,
    }
}

pub const RCC_BASE: u32 = PERIPH_BASE + 0x00000000;
pub static RCC: Rcc = Rcc::new(RCC_BASE);

pub const RCC_CR0_STCLKEN_SEL_MASK: u32 = 0x02000000;
pub const RCC_CR0_STCLKEN_SEL_XO32K: u32 = 0x00000000;
pub const RCC_CR0_STCLKEN_SEL_RCO32K: u32 = 0x02000000;

pub const RCC_CR0_MCO_CLK_DIV_MASK: u32 = 0x01C00000;
pub const RCC_CR0_MCO_CLK_DIV_1: u32 = 0x00000000;
pub const RCC_CR0_MCO_CLK_DIV_2: u32 = 0x01000000;
pub const RCC_CR0_MCO_CLK_DIV_4: u32 = 0x01400000;
pub const RCC_CR0_MCO_CLK_DIV_8: u32 = 0x01800000;
pub const RCC_CR0_MCO_CLK_DIV_16: u32 = 0x1C00000;

pub const RCC_CR0_MCO_CLK_SEL_MASK: u32 = 0x00380000;
pub const RCC_CR0_MCO_CLK_SEL_RCO32K: u32 = 0x00000000;
pub const RCC_CR0_MCO_CLK_SEL_XO32K: u32 = 0x00080000;
pub const RCC_CR0_MCO_CLK_SEL_RCO4M: u32 = 0x00100000;
pub const RCC_CR0_MCO_CLK_SEL_XO24M: u32 = 0x00180000;
pub const RCC_CR0_MCO_CLK_SEL_XO32M: u32 = 0x00200000;
pub const RCC_CR0_MCO_CLK_SEL_RCO48M: u32 = 0x00280000;
pub const RCC_CR0_MCO_CLK_SEL_PLL: u32 = 0x00300000;
pub const RCC_CR0_MCO_CLK_SEL_SYSCLCK: u32 = 0x00380000;

pub const RCC_CR0_MCO_CLK_OUT_EN_MASK: u32 = 0x00040000;

pub const RCC_CR0_PCLK1_DIV_MASK: u32 = 0x00038000;
pub const RCC_CR0_PCLK1_DIV_1: u32 = 0x00000000;
pub const RCC_CR0_PCLK1_DIV_2: u32 = 0x00008000;
pub const RCC_CR0_PCLK1_DIV_4: u32 = 0x00010000;
pub const RCC_CR0_PCLK1_DIV_8: u32 = 0x00018000;
pub const RCC_CR0_PCLK1_DIV_16: u32 = 0x00020000;

pub const RCC_CR0_SYSCLK_SEL_MASK: u32 = 0x00007000;
pub const RCC_CR0_SYSCLK_SEL_RCO48M_DIV2: u32 = 0x00000000;
pub const RCC_CR0_SYSCLK_SEL_RCO32K: u32 = 0x00001000;
pub const RCC_CR0_SYSCLK_SEL_XO32K: u32 = 0x00002000;
pub const RCC_CR0_SYSCLK_SEL_PLL: u32 = 0x00003000;
pub const RCC_CR0_SYSCLK_SEL_XO24M: u32 = 0x00004000;
pub const RCC_CR0_SYSCLK_SEL_XO32M: u32 = 0x00005000;
pub const RCC_CR0_SYSCLK_SEL_RCO4M: u32 = 0x00006000;
pub const RCC_CR0_SYSCLK_SEL_RCO48M: u32 = 0x00007000;

pub const RCC_CR0_HCLK_DIV_MASK: u32 = 0x00000F00;
pub const RCC_CR0_HCLK_DIV_1: u32 = 0x00000000;
pub const RCC_CR0_HCLK_DIV_2: u32 = 0x00000100;
pub const RCC_CR0_HCLK_DIV_4: u32 = 0x00000200;
pub const RCC_CR0_HCLK_DIV_8: u32 = 0x00000300;
pub const RCC_CR0_HCLK_DIV_16: u32 = 0x00000400;
pub const RCC_CR0_HCLK_DIV_32: u32 = 0x00000500;
pub const RCC_CR0_HCLK_DIV_64: u32 = 0x00000600;
pub const RCC_CR0_HCLK_DIV_128: u32 = 0x00000700;
pub const RCC_CR0_HCLK_DIV_256: u32 = 0x00000800;
pub const RCC_CR0_HCLK_DIV_512: u32 = 0x00000900;

pub const RCC_CR0_PCLK0_DIV_MASK: u32 = 0x000000E0;
pub const RCC_CR0_PCLK0_DIV_1: u32 = 0x00000000;
pub const RCC_CR0_PCLK0_DIV_2: u32 = 0x00000020;
pub const RCC_CR0_PCLK0_DIV_4: u32 = 0x00000040;
pub const RCC_CR0_PCLK0_DIV_8: u32 = 0x00000060;
pub const RCC_CR0_PCLK0_DIV_16: u32 = 0x00000080;

pub const RCC_CR1_LPTIMER1_EXTCLK_SEL_MASK: u32 = 0x00000800;

pub const RCC_CR1_LPTIMER0_EXTCLK_SEL_MASK: u32 = 0x00000400;

pub const RCC_CR1_LPTIMER1_CLK_SEL_MASK: u32 = 0x00000300;
pub const RCC_CR1_LPTIMER1_CLK_SEL_PCLK0: u32 = 0x00000000;
pub const RCC_CR1_LPTIMER1_CLK_SEL_RCO4M: u32 = 0x00000100;
pub const RCC_CR1_LPTIMER1_CLK_SEL_XO32K: u32 = 0x00000200;
pub const RCC_CR1_LPTIMER1_CLK_SEL_RCO32K: u32 = 0x00000300;

pub const RCC_CR1_LPTIMER0_CLK_SEL_MASK: u32 = 0x000000C0;
pub const RCC_CR1_LPTIMER0_CLK_SEL_PCLK0: u32 = 0x00000000;
pub const RCC_CR1_LPTIMER0_CLK_SEL_RCO4M: u32 = 0x00000040;
pub const RCC_CR1_LPTIMER0_CLK_SEL_XO32K: u32 = 0x00000080;
pub const RCC_CR1_LPTIMER0_CLK_SEL_RCO32K: u32 = 0x000000C0;

pub const RCC_CR1_LCD_CLK_SEL_MASK: u32 = 0x00000030;
pub const RCC_CR1_LCD_CLK_SEL_XO32K: u32 = 0x00000000;
pub const RCC_CR1_LCD_CLK_SEL_RCO32K: u32 = 0x00000010;
pub const RCC_CR1_LCD_CLK_SEL_RCO4M: u32 = 0x00000020;

pub const RCC_CR1_LPUART_CLK_SEL_MASK: u32 = 0x0000000C;
pub const RCC_CR1_LPUART_CLK_SEL_XO32K: u32 = 0x00000000;
pub const RCC_CR1_LPUART_CLK_SEL_RCO32K: u32 = 0x00000004;
pub const RCC_CR1_LPUART_CLK_SEL_RCO4M: u32 = 0x00000008;

pub const RCC_CR1_RTC_CLK_SEL_MASK: u32 = 0x00000002;
pub const RCC_CR1_RTC_CLK_SEL_XO32K: u32 = 0x00000000;
pub const RCC_CR1_RTC_CLK_SEL_RCO32K: u32 = 0x00000002;

pub const RCC_CR1_IWDG_CLK_SEL_MASK: u32 = 0x00000001;
pub const RCC_CR1_IWDG_CLK_SEL_XO32K: u32 = 0x00000000;
pub const RCC_CR1_IWDG_CLK_SEL_RCO32K: u32 = 0x00000001;

pub const RCC_CR2_UART0_CLK_SEL_MASK: u32 = 0x00018000;
pub const RCC_CR2_UART0_CLK_SEL_PCLK0: u32 = 0x00000000;
pub const RCC_CR2_UART0_CLK_SEL_RCO4M: u32 = 0x00008000;
pub const RCC_CR2_UART0_CLK_SEL_XO32K: u32 = 0x00010000;
pub const RCC_CR2_UART0_CLK_SEL_XO24M: u32 = 0x00018000;

pub const RCC_CR2_UART1_CLK_SEL_MASK: u32 = 0x00006000;
pub const RCC_CR2_UART1_CLK_SEL_PCLK0: u32 = 0x00000000;
pub const RCC_CR2_UART1_CLK_SEL_RCO4M: u32 = 0x00002000;
pub const RCC_CR2_UART1_CLK_SEL_XO32K: u32 = 0x00004000;
pub const RCC_CR2_UART1_CLK_SEL_XO24M: u32 = 0x00006000;

pub const RCC_CR2_UART2_CLK_SEL_MASK: u32 = 0x00001800;
pub const RCC_CR2_UART2_CLK_SEL_PCLK1: u32 = 0x00000000;
pub const RCC_CR2_UART2_CLK_SEL_RCO4M: u32 = 0x00000800;
pub const RCC_CR2_UART2_CLK_SEL_XO32K: u32 = 0x00001000;
pub const RCC_CR2_UART2_CLK_SEL_XO24M: u32 = 0x00001800;

pub const RCC_CR2_UART3_CLK_SEL_MASK: u32 = 0x00000600;
pub const RCC_CR2_UART3_CLK_SEL_PCLK1: u32 = 0x00000000;
pub const RCC_CR2_UART3_CLK_SEL_RCO4M: u32 = 0x00000200;
pub const RCC_CR2_UART3_CLK_SEL_XO32K: u32 = 0x00000400;
pub const RCC_CR2_UART3_CLK_SEL_XO24M: u32 = 0x00000600;

pub const RCC_CR2_SCC_CLK_SEL_MASK: u32 = 0x00000180;
pub const RCC_CR2_SCC_CLK_SEL_PCLK1: u32 = 0x00000000;
pub const RCC_CR2_SCC_CLK_SEL_SYSCLK: u32 = 0x00000080;
pub const RCC_CR2_SCC_CLK_SEL_PLL: u32 = 0x00000100;
pub const RCC_CR2_SCC_CLK_SEL_EXT: u32 = 0x00000180;

pub const RCC_CR2_ADC_CLK_SEL_MASK: u32 = 0x00000060;
pub const RCC_CR2_ADC_CLK_SEL_PCLK1: u32 = 0x00000000;
pub const RCC_CR2_ADC_CLK_SEL_SYSCLK: u32 = 0x00000020;
pub const RCC_CR2_ADC_CLK_SEL_PLL: u32 = 0x00000040;
pub const RCC_CR2_ADC_CLK_SEL_RCO48M: u32 = 0x00000060;

pub const RCC_CR2_I2S_CLK_SEL_MASK: u32 = 0x0000001C;
pub const RCC_CR2_I2S_CLK_SEL_PCLK0: u32 = 0x00000000;
pub const RCC_CR2_I2S_CLK_SEL_XO24M: u32 = 0x00000004;
pub const RCC_CR2_I2S_CLK_SEL_PLL: u32 = 0x00000008;
pub const RCC_CR2_I2S_CLK_SEL_XO32M: u32 = 0x0000000C;
pub const RCC_CR2_I2S_CLK_SEL_EXT_CLK: u32 = 0x00000010;

pub const RCC_CR2_QSPI_CLK_SEL_MASK: u32 = 0x00000003;
pub const RCC_CR2_QSPI_CLK_SEL_HCLK: u32 = 0x00000000;
pub const RCC_CR2_QSPI_CLK_SEL_SYSCLK: u32 = 0x00000001;
pub const RCC_CR2_QSPI_CLK_SEL_PLL: u32 = 0x00000002;

pub const RCC_CR3_I2S_MCLK_DIV_MASK: u32 = 0x0000FF00;

pub const RCC_CR3_I2S_SCLK_DIV_MASK: u32 = 0x000000FF;

pub const RCC_CGR0_PWR_CLK_EN_MASK: u32 = 0x80000000;
pub const RCC_CGR0_DMAC0_CLK_EN_MASK: u32 = 0x40000000;
pub const RCC_CGR0_DMAC1_CLK_EN_MASK: u32 = 0x20000000;
pub const RCC_CGR0_CRC_CLK_EN_MASK: u32 = 0x10000000;
pub const RCC_CGR0_BSTIMER0_CLK_EN_MASK: u32 = 0x08000000;
pub const RCC_CGR0_BSTIMER1_CLK_EN_MASK: u32 = 0x04000000;
pub const RCC_CGR0_IOM0_CLK_EN_MASK: u32 = 0x02000000;
pub const RCC_CGR0_IOM1_CLK_EN_MASK: u32 = 0x01000000;
pub const RCC_CGR0_IOM2_CLK_EN_MASK: u32 = 0x00800000;
pub const RCC_CGR0_IOM3_CLK_EN_MASK: u32 = 0x00400000;
pub const RCC_CGR0_SYSCFG_CLK_EN_MASK: u32 = 0x00200000;
pub const RCC_CGR0_UART0_CLK_EN_MASK: u32 = 0x00100000;
pub const RCC_CGR0_UART1_CLK_EN_MASK: u32 = 0x00080000;
pub const RCC_CGR0_UART2_CLK_EN_MASK: u32 = 0x00040000;
pub const RCC_CGR0_UART3_CLK_EN_MASK: u32 = 0x00020000;
pub const RCC_CGR0_LPUART_CLK_EN_MASK: u32 = 0x00010000;
pub const RCC_CGR0_SSP0_CLK_EN_MASK: u32 = 0x00008000;
pub const RCC_CGR0_SSP1_CLK_EN_MASK: u32 = 0x00004000;
pub const RCC_CGR0_SSP2_CLK_EN_MASK: u32 = 0x00002000;
pub const RCC_CGR0_I2C0_CLK_EN_MASK: u32 = 0x00001000;
pub const RCC_CGR0_I2C1_CLK_EN_MASK: u32 = 0x00000800;
pub const RCC_CGR0_I2C2_CLK_EN_MASK: u32 = 0x00000400;
pub const RCC_CGR0_SCC_CLK_EN_MASK: u32 = 0x00000200;
pub const RCC_CGR0_ADC_CLK_EN_MASK: u32 = 0x00000100;
pub const RCC_CGR0_AFEC_CLK_EN_MASK: u32 = 0x00000080;
pub const RCC_CGR0_LCD_CLK_EN_MASK: u32 = 0x00000040;
pub const RCC_CGR0_DAC_CLK_EN_MASK: u32 = 0x00000020;
pub const RCC_CGR0_LORA_CLK_EN_MASK: u32 = 0x00000010;
pub const RCC_CGR0_TIMER0_CLK_EN_MASK: u32 = 0x00000008;
pub const RCC_CGR0_TIMER1_CLK_EN_MASK: u32 = 0x00000004;
pub const RCC_CGR0_TIMER2_CLK_EN_MASK: u32 = 0x00000002;
pub const RCC_CGR0_TIMER3_CLK_EN_MASK: u32 = 0x00000001;

pub const RCC_CGR1_LPTIMER1_PCLK_EN_MASK: u32 = 0x00001000;
pub const RCC_CGR1_LPTIMER1_CLK_EN_MASK: u32 = 0x00000800;
pub const RCC_CGR1_RNGC_CLK_EN_MASK: u32 = 0x00000400;
pub const RCC_CGR1_LPTIMER0_PCLK_EN_MASK: u32 = 0x00000200;
pub const RCC_CGR1_I2S_CLK_EN_MASK: u32 = 0x00000100;
pub const RCC_CGR1_SAC_CLK_EN_MASK: u32 = 0x00000080;
pub const RCC_CGR1_WDG_CNT_CLK_EN_MASK: u32 = 0x00000040;
pub const RCC_CGR1_QSPI_CLK_EN_MASK: u32 = 0x00000020;
pub const RCC_CGR1_LPTIMER0_CLK_EN_MASK: u32 = 0x00000010;
pub const RCC_CGR1_IWDG_CLK_EN_MASK: u32 = 0x00000008;
pub const RCC_CGR1_WDG_CLK_EN_MASK: u32 = 0x00000004;
pub const RCC_CGR1_RTC_CLK_EN_MASK: u32 = 0x00000002;
pub const RCC_CGR1_SEC_CLK_EN_MASK: u32 = 0x00000001;

pub const RCC_CGR2_LPTIMER1_AON_CLK_EN_MASK: u32 = 0x00000020;
pub const RCC_CGR2_LPTIMER0_AON_CLK_EN_MASK: u32 = 0x00000010;
pub const RCC_CGR2_LCD_AON_CLK_EN_MASK: u32 = 0x00000008;
pub const RCC_CGR2_LPUART_AON_CLK_EN_MASK: u32 = 0x00000004;
pub const RCC_CGR2_RTC_AON_CLK_EN_MASK: u32 = 0x00000002;
pub const RCC_CGR2_IWDG_CLK_EN_MASK: u32 = 0x00000001;

pub const RCC_RST0_UART0_RST_N_MASK: u32 = 0x80000000;
pub const RCC_RST0_UART1_RST_N_MASK: u32 = 0x40000000;
pub const RCC_RST0_UART2_RST_N_MASK: u32 = 0x20000000;
pub const RCC_RST0_UART3_RST_N_MASK: u32 = 0x10000000;
pub const RCC_RST0_LPUART_RST_N_MASK: u32 = 0x08000000;
pub const RCC_RST0_SSP0_RST_N_MASK: u32 = 0x04000000;
pub const RCC_RST0_SSP1_RST_N_MASK: u32 = 0x02000000;
pub const RCC_RST0_SSP2_RST_N_MASK: u32 = 0x01000000;
pub const RCC_RST0_QSPI_RST_N_MASK: u32 = 0x00800000;
pub const RCC_RST0_I2C0_RST_N_MASK: u32 = 0x00400000;
pub const RCC_RST0_I2C1_RST_N_MASK: u32 = 0x00200000;
pub const RCC_RST0_I2C2_RST_N_MASK: u32 = 0x00100000;
pub const RCC_RST0_SCC_RST_N_MASK: u32 = 0x00080000;
pub const RCC_RST0_ADC_RST_N_MASK: u32 = 0x00040000;
pub const RCC_RST0_AFEC_RST_N_MASK: u32 = 0x00020000;
pub const RCC_RST0_LCD_RST_N_MASK: u32 = 0x00010000;
pub const RCC_RST0_DAC_RST_N_MASK: u32 = 0x00008000;
pub const RCC_RST0_LORA_RST_N_MASK: u32 = 0x00004000;
pub const RCC_RST0_IOM_RST_N_MASK: u32 = 0x00002000;
pub const RCC_RST0_TIMER0_RST_N_MASK: u32 = 0x00001000;
pub const RCC_RST0_TIMER1_RST_N_MASK: u32 = 0x00000800;
pub const RCC_RST0_TIMER2_RST_N_MASK: u32 = 0x00000400;
pub const RCC_RST0_TIMER3_RST_N_MASK: u32 = 0x00000200;
pub const RCC_RST0_BSTIMER0_RST_N_MASK: u32 = 0x00000100;
pub const RCC_RST0_BSTIMER1_RST_N_MASK: u32 = 0x00000080;
pub const RCC_RST0_LPTIMER0_RST_N_MASK: u32 = 0x00000040;
pub const RCC_RST0_IWDG_RST_N_MASK: u32 = 0x00000020;
pub const RCC_RST0_WDG_RST_N_MASK: u32 = 0x00000010;
pub const RCC_RST0_RTC_RST_N_MASK: u32 = 0x00000008;
pub const RCC_RST0_CRC_RST_N_MASK: u32 = 0x00000004;
pub const RCC_RST0_SEC_RST_N_MASK: u32 = 0x00000002;
pub const RCC_RST0_SAC_RST_N_MASK: u32 = 0x00000001;

pub const RCC_RST1_LPTIMER1_RST_N_MASK: u32 = 0x00000010;
pub const RCC_RST1_RNGC_RST_N_MASK: u32 = 0x00000008;
pub const RCC_RST1_I2S_RST_N_MASK: u32 = 0x00000004;
pub const RCC_RST1_DMAC0_RST_N_MASK: u32 = 0x00000002;
pub const RCC_RST1_DMAC1_RST_N_MASK: u32 = 0x00000001;

pub const RCC_RST_SR_BOR_RESET_SR: u32 = 0x00000040;
pub const RCC_RST_SR_IWDG_RESET_SR: u32 = 0x00000020;
pub const RCC_RST_SR_WDG_RESET_SR: u32 = 0x00000010;
pub const RCC_RST_SR_EFC_RESET_SR: u32 = 0x00000008;
pub const RCC_RST_SR_CPU_RESET_SR: u32 = 0x00000004;
pub const RCC_RST_SR_SEC_RESET_SR: u32 = 0x00000002;
pub const RCC_RST_SR_STANDBY_RESET_SR: u32 = 0x00000001;

pub const RCC_RST_CR_RESET_REQ_EN_MASK: u32 = 0x0000003E;
pub const RCC_RST_CR_IWDG_RESET_REQ_EN_MASK: u32 = 0x00000020;
pub const RCC_RST_CR_WDG_RESET_REQ_EN_MASK: u32 = 0x00000010;
pub const RCC_RST_CR_EFC_RESET_REQ_EN_MASK: u32 = 0x00000008;
pub const RCC_RST_CR_CPU_RESET_REQ_EN_MASK: u32 = 0x00000004;
pub const RCC_RST_CR_SEC_RESET_REQ_EN_MASK: u32 = 0x00000002;

pub const RCC_SR_ALL_DONE: u32 = 0x0000003F;
pub const RCC_SR_LPTIMER1_AON_CLK_EN_DONE: u32 = 0x00000020;
pub const RCC_SR_LPTIM_AON_CLK_EN_DONE: u32 = 0x00000010;
pub const RCC_SR_LCD_AON_CLK_EN_DONE: u32 = 0x00000008;
pub const RCC_SR_LPUART_AON_CLK_EN_DONE: u32 = 0x00000004;
pub const RCC_SR_RTC_AON_CLK_EN_DONE: u32 = 0x00000002;
pub const RCC_SR_IWDG_AON_CLK_EN_DONE: u32 = 0x00000001;

pub const RCC_SR1_LPTIMER1_CLK_EN_SYNC: u32 = 0x00100000;
pub const RCC_SR1_LPTIMER1_AON_CLK_EN_SYNC: u32 = 0x00080000;
pub const RCC_SR1_UART0_CLK_EN_SYNC: u32 = 0x00040000;
pub const RCC_SR1_UART1_CLK_EN_SYNC: u32 = 0x00020000;
pub const RCC_SR1_UART2_CLK_EN_SYNC: u32 = 0x00010000;
pub const RCC_SR1_UART3_CLK_EN_SYNC: u32 = 0x00008000;
pub const RCC_SR1_SCC_CLK_EN_SYNC: u32 = 0x00004000;
pub const RCC_SR1_ADC_CLK_EN_SYNC: u32 = 0x00002000;
pub const RCC_SR1_LPTIMER0_CLK_EN_SYNC: u32 = 0x00001000;
pub const RCC_SR1_QSPI_CLK_EN_SYNC: u32 = 0x00000800;
pub const RCC_SR1_LPUART_CLK_EN_SYNC: u32 = 0x00000400;
pub const RCC_SR1_LCD_CLK_EN_SYNC: u32 = 0x00000200;
pub const RCC_SR1_IWDG_CLK_EN_SYNC: u32 = 0x00000100;
pub const RCC_SR1_RTC_CLK_EN_SYNC: u32 = 0x00000080;
pub const RCC_SR1_MCO_CLK_EN_SYNC: u32 = 0x00000040;
pub const RCC_SR1_I2S_CLK_EN_SYNC: u32 = 0x00000020;
pub const RCC_SR1_LPTIMER0_AON_CLK_EN_SYNC: u32 = 0x00000010;
pub const RCC_SR1_LCD_AON_CLK_EN_SYNC: u32 = 0x00000008;
pub const RCC_SR1_LPUART_AON_CLK_EN_SYNC: u32 = 0x00000004;
pub const RCC_SR1_RTC_AON_CLK_EN_SYNC: u32 = 0x00000002;
pub const RCC_SR1_IWDG_AON_CLK_EN_SYNC: u32 = 0x00000001;

define_reg! {
    Ssp
    __Ssp {
        cr0: u32,
        cr1: u32,
        dr: u32,
        sr: u32,
        cpsr: u32,
        imsc: u32,
        ris: u32,
        mis: u32,
        icr: u32,
        dma_cr: u32,
        resv: [u32; 1006],
        periph_id0: u32,
        periph_id1: u32,
        periph_id2: u32,
        periph_id3: u32,
        pcell_id0: u32,
        pcell_id1: u32,
        pcell_id2: u32,
        pcell_id3: u32,
    }
}

pub const SSP0_BASE: u32 = PERIPH_BASE + 0x00006000;
pub const SSP1_BASE: u32 = PERIPH_BASE + 0x00012000;
pub const SSP2_BASE: u32 = PERIPH_BASE + 0x00013000;
pub static SSP0: Ssp = Ssp::new(SSP0_BASE);
pub static SSP1: Ssp = Ssp::new(SSP1_BASE);
pub static SSP2: Ssp = Ssp::new(SSP2_BASE);

pub const SSP_NUM_PORTS: u32 = 3;

define_reg! {
    Gpio
    __Gpio {
        oer: u32,
        otyper: u32,
        ier: u32,
        per: u32,
        psr: u32,
        idr: u32,
        odr: u32,
        brr: u32,
        bsr: u32,
        dsr: u32,
        icr: u32,
        ifr: u32,
        wucr: u32,
        wulvl: u32,
        afrl: u32,
        afrh: u32,
        stop3_wucr: u32,
    }
}

pub const GPIO_BASE: u32 = 0x4001F000;
pub const GPIOA_BASE: u32 = GPIO_BASE;
pub const GPIOB_BASE: u32 = GPIO_BASE + 0x400;
pub const GPIOC_BASE: u32 = GPIO_BASE + 0x800;
pub const GPIOD_BASE: u32 = GPIO_BASE + 0xC00;
pub static GPIOA: Gpio = Gpio::new(GPIOA_BASE);
pub static GPIOB: Gpio = Gpio::new(GPIOB_BASE);
pub static GPIOC: Gpio = Gpio::new(GPIOC_BASE);
pub static GPIOD: Gpio = Gpio::new(GPIOD_BASE);

define_reg! {
    Rtc
    __Rtc {
        ctrl: u32,
        alarm0: u32,
        alarm1: u32,
        ppm_adjust: u32,
        calendar: u32,
        calendar_h: u32,
        cyc_max: u32,
        sr: u32,
        asyn_data: u32,
        asyn_data_h: u32,
        cr1: u32,
        sr1: u32,
        cr2: u32,
        sub_second_cnt: u32,
        cyc_cnt: u32,
        alarm0_subsecond: u32,
        alarm1_subsecond: u32,
        calendar_r: u32,
        calendar_r_h: u32,
    }
}

pub const RTC_REG_BASE: u32 = 0x4000E000;
pub static RTC: Rtc = Rtc::new(RTC_REG_BASE);

define_reg! {
    Uart
    __Uart {
        dr: u32,
        rsc_ecr: u32,
        rsv0: [u32; 4],
        fr: u32,
        rsv1: u32,
        ilpr: u32,
        ibrd: u32,
        fbrd: u32,
        lcr_h: u32,
        cr: u32,
        ifls: u32,
        imsc: u32,
        ris: u32,
        mis: u32,
        icr: u32,
        dmacr: u32,
        rsv2: [u32; 997],
        id: [u32; 8],
    }
}

pub const UART0_BASE: u32 = PERIPH_BASE + 0x3000;
pub const UART1_BASE: u32 = PERIPH_BASE + 0x4000;
pub const UART2_BASE: u32 = PERIPH_BASE + 0x10000;
pub const UART3_BASE: u32 = PERIPH_BASE + 0x11000;

pub static UART0: Uart = Uart::new(UART0_BASE);
pub static UART1: Uart = Uart::new(UART1_BASE);
pub static UART2: Uart = Uart::new(UART2_BASE);
pub static UART3: Uart = Uart::new(UART3_BASE);

/****************************UART CR bit definition***************************/
pub const UART_CR_SIR_LPIRDA_EN: u32 = 0x00000004;
pub const UART_CR_SIR_EN: u32 = 0x00000002;

pub const UART_CR_UART_EN: u32 = 0x00000001;

pub const UART_CR_UART_MODE: u32 = 0x00000030;
pub const UART_CR_UART_MODE_NONE: u32 = 0x00000000;
pub const UART_CR_UART_MODE_RX: u32 = 0x00000020;
pub const UART_CR_UART_MODE_TX: u32 = 0x00000010;
pub const UART_CR_UART_MODE_TXRX: u32 = 0x00000030;

pub const UART_CR_FLOW_CTRL: u32 = 0x0000C000;
pub const UART_CR_FLOW_CTRL_NONE: u32 = 0x00000000;
pub const UART_CR_FLOW_CTRL_CTS: u32 = 0x00008000;
pub const UART_CR_FLOW_CTRL_RTS: u32 = 0x00004000;
pub const UART_CR_FLOW_CTRL_CTS_RTS: u32 = 0x0000C000;

/****************************UART LCR_H bit definition*************************/
pub const UART_LCR_H_PEN: u32 = 0x00000002;

pub const UART_LCR_H_EPS_EVEN: u32 = 0x00000004;

pub const UART_LCR_H_STOP: u32 = 0x00000008;
pub const UART_LCR_H_STOP_1: u32 = 0x00000000;
pub const UART_LCR_H_STOP_2: u32 = 0x00000008;

pub const UART_LCR_H_FEN: u32 = 0x00000010;

pub const UART_LCR_H_WLEN: u32 = 0x00000060;
pub const UART_LCR_H_WLEN_5: u32 = 0x00000000;
pub const UART_LCR_H_WLEN_6: u32 = 0x00000020;
pub const UART_LCR_H_WLEN_7: u32 = 0x00000040;
pub const UART_LCR_H_WLEN_8: u32 = 0x00000060;

/****************************UART IFLS bit definition**************************/
pub const UART_IFLS_TX: u32 = 0x00000007;
pub const UART_IFLS_TX_1_8: u32 = 0x00000000;
pub const UART_IFLS_TX_1_4: u32 = 0x00000001;
pub const UART_IFLS_TX_1_2: u32 = 0x00000002;
pub const UART_IFLS_TX_3_4: u32 = 0x00000003;
pub const UART_IFLS_TX_7_8: u32 = 0x00000004;

pub const UART_IFLS_RX: u32 = 0x00000038;
pub const UART_IFLS_RX_1_8: u32 = 0x00000000;
pub const UART_IFLS_RX_1_4: u32 = 0x00000008;
pub const UART_IFLS_RX_1_2: u32 = 0x00000010;
pub const UART_IFLS_RX_3_4: u32 = 0x00000018;
pub const UART_IFLS_RX_7_8: u32 = 0x00000020;

/****************************UART DMACR bit definition*************************/
pub const UART_DMACR_ONERR_EN_MASK: u32 = 0x00000004;

pub const UART_DMACR_TX_EN_MASK: u32 = 0x00000002;

pub const UART_DMACR_RX_EN_MASK: u32 = 0x00000001;

define_reg! {
    Lpuart
    __Lpuart {
        cr0: u32,
        cr1: u32,
        sr0: u32,
        sr1: u32,
        data: u32,
    }
}

pub const LPUART_BASE: u32 = PERIPH_BASE + 0x5000;
pub static LPUART: Lpuart = Lpuart::new(LPUART_BASE);

define_reg! {
    Efc
    __Efc {
        cr: u32,
        int_en: u32,
        sr: u32,
        program_data0: u32,
        program_data1: u32,
        timing_cfg: u32,
        protect_seq: u32,
        rsv0: u32,
        chip_pattern: u32,
        ip_trim_l: u32,
        ip_trim_h: u32,
        sn_l: u32,
        sn_h: u32,
        test_info_l: u32,
        test_info_h: u32,
        option_csr_bytes: u32,
        option_e0_bytes: u32,
        option_wp_bytes: u32,
        option_sec_bytes0: u32,
        option_sec_bytes1: u32,
    }
}

pub const EFC_BASE: u32 = PERIPH_BASE + 0x20000;
pub static EFC: Efc = Efc::new(EFC_BASE);

/****************************EFC CR bit definition*****************************/
pub const EFC_CR_INFO_LOAD_MASK: u32 = 0x80000000;

pub const EFC_CR_ECC_DISABLE_MASK: u32 = 0x00000200;

pub const EFC_CR_OPTION_OP_EN_MASK: u32 = 0x00000100;

pub const EFC_CR_FACTORY_OP_EN_MASK: u32 = 0x00000080;

pub const EFC_CR_WRITE_RELEASE_EN_MASK: u32 = 0x00000040;

pub const EFC_CR_PREFETCH_EN_MASK: u32 = 0x00000020;

pub const EFC_CR_READ_ACC_EN_MASK: u32 = 0x00000010;

pub const EFC_CR_PROG_MODE_MASK: u32 = 0x00000008;
pub const EFC_CR_PROG_MODE_DWORD: u32 = 0x00000000;
pub const EFC_CR_PROG_MODE_WLINE: u32 = 0x00000008;

pub const EFC_CR_PROG_EN_MASK: u32 = 0x00000004;

pub const EFC_CR_PAGE_ERASE_EN_MASK: u32 = 0x00000002;

pub const EFC_CR_MASS_ERASE_EN_MASK: u32 = 0x00000001;

/****************************EFC INT_EN bit definition*************************/

/****************************EFC TIMING_CFG bit definition*********************/
pub const EFC_TIMING_CFG_READ_NUM_MASK: u32 = 0x000F0000;

/****************************EFC SR bit definition*****************************/
pub const EFC_SR_OPTION_WRITE_ERROR: u32 = 0x00000010;

pub const EFC_SR_PROGRAM_DATA_WAIT: u32 = 0x00000004;

pub const EFC_SR_READ_NUM_DONE: u32 = 0x00000002;

pub const EFC_SR_OPERATION_DONE: u32 = 0x00000001;

define_reg! {
    Lorac
    __Lorac {
        ssp_cr0: u32,
        ssp_cr1: u32,
        ssp_dr: u32,
        ssp_sr: u32,
        ssp_cpsr: u32,
        ssp_imsc: u32,
        ssp_ris: u32,
        ssp_mis: u32,
        ssp_icr: u32,
        ssp_dma_cr: u32,
        rsv: [u32; 54],
        cr0: u32,
        cr1: u32,
        sr: u32,
        nss_cr: u32,
        sck_cr: u32,
        mosi_cr: u32,
        miso_sr: u32,
    }
}

pub const LORAC_BASE: u32 = PERIPH_BASE + 0x9000;
pub static LORAC: Lorac = Lorac::new(LORAC_BASE);
define_reg! {
    Afec
    __Afec {
        cr: u32,
        int_sr: u32,
        raw_sr: u32,
    }
}
pub static AFEC: Afec = Afec::new(AFEC_BASE + 0x200);

pub const AFEC_RAW_SR_RCO4M_READY_MASK: u32 = 0x80000000;

pub const AFEC_RAW_SR_PLL_UNLOCK_MASK: u32 = 0x40000000;

pub const AFEC_RAW_SR_RCO24M_READY_MASK: u32 = 0x00000004;
define_reg! {
    Iwdg
    __Iwdg {
        cr: u32,
        max: u32,
        win: u32,
        sr: u32,
        sr1: u32,
        cr1: u32,
        sr2: u32,
    }
}

pub const IWDG_BASE: u32 = PERIPH_BASE + 0x1D000;
pub static IWDG: Iwdg = Iwdg::new(IWDG_BASE);

/****************************IWDG CR bit definition*************************/
pub const IWDG_CR_RSTEN_MASK: u32 = 0x00000020;
pub const IWDG_CR_WKEN_MASK: u32 = 0x00000010;

pub const IWDG_CR_PREDIV_MASK: u32 = 0x0000000E;
pub const IWDG_CR_PREDIV_4: u32 = 0x00000000;
pub const IWDG_CR_PREDIV_8: u32 = 0x00000002;
pub const IWDG_CR_PREDIV_16: u32 = 0x00000004;
pub const IWDG_CR_PREDIV_32: u32 = 0x00000006;
pub const IWDG_CR_PREDIV_64: u32 = 0x00000008;
pub const IWDG_CR_PREDIV_128: u32 = 0x0000000A;
pub const IWDG_CR_PREDIV_256: u32 = 0x0000000C;

pub const IWDG_CR_START_MASK: u32 = 0x00000001;

/****************************IWDG SR bit definition*************************/
pub const IWDG_SR_WRITE_SR2_DONE: u32 = 0x00000008;
pub const IWDG_SR_WIN_SET_DONE: u32 = 0x00000004;
pub const IWDG_SR_MAX_SET_DONE: u32 = 0x00000002;
pub const IWDG_SR_WRITE_CR_DONE: u32 = 0x00000001;

/****************************IWDG SR1 bit definition************************/
pub const IWDG_SR1_RESET_REQ_SYNC: u32 = 0x00001000;

/****************************IWDG CR1 bit definition************************/
pub const IWDG_CR1_RESET_REQ_RST_EN_MASK: u32 = 0x00000002;
pub const IWDG_CR1_RESET_REQ_INT_EN_MASK: u32 = 0x00000001;

/****************************IWDG SR2 bit definition************************/
pub const IWDG_SR2_RESET_REQ_SR_MASK: u32 = 0x00000001;

define_reg! {
    Wdg
    __Wdg {
        load: u32,
        value: u32,
        control: u32,
        intclr: u32,
        ris: u32,
        mis: u32,
        dummy0: [u32; 0x2FA],
        lock: u32,
        dummy1: [u32; 0xBF],
        itcr: u32,
        itop: u32,
        dummy2: [u32; 0x32],
        periphid4: u32,
        periphid5: u32,
        periphid6: u32,
        periphid7: u32,
        periphid0: u32,
        periphid1: u32,
        periphid2: u32,
        periphid3: u32,
        pcellid0: u32,
        pcellid1: u32,
        pcellid2: u32,
        pcellid3: u32,
    }
}

pub const WDG_BASE: u32 = PERIPH_BASE + 0x1E000;
pub static WDG: Wdg = Wdg::new(WDG_BASE);

define_reg! {
    Crc
    __Crc {
        cr: u32,
        dr: u32,
        init: u32,
        poly: u32,
    }
}

/// CRC base address
pub const CRC_BASE: u32 = PERIPH_BASE + 0x22000;
/// CRC peripheral
pub static CRC: Crc = Crc::new(CRC_BASE);

/****************************CRC CR bit definition*****************************/
pub const CRC_CR_CALC_FLAG: u32 = 0x00000040;

pub const CRC_CR_CALC_INIT: u32 = 0x00000020;

pub const CRC_CR_POLY_SIZE_MASK: u32 = 0x00000018;
pub const CRC_CR_POLY_SIZE_7: u32 = 0x00000018;
pub const CRC_CR_POLY_SIZE_8: u32 = 0x00000010;
pub const CRC_CR_POLY_SIZE_16: u32 = 0x00000008;
pub const CRC_CR_POLY_SIZE_32: u32 = 0x00000000;

pub const CRC_CR_REVERSE_IN_MASK: u32 = 0x00000006;
pub const CRC_CR_REVERSE_IN_NONE: u32 = 0x00000000;
pub const CRC_CR_REVERSE_IN_BYTE: u32 = 0x00000002;
pub const CRC_CR_REVERSE_IN_HWORD: u32 = 0x00000004;
pub const CRC_CR_REVERSE_IN_WORD: u32 = 0x00000006;

pub const CRC_CR_REVERSE_OUT_EN: u32 = 0x00000001;
define_reg! {
    I2c
    __I2c {
        cr: u32,
        sr: u32,
        sar: u32,
        dbr: u32,
        lcr: u32,
        wcr: u32,
        rst_cycl: u32,
        bmr: u32,
        wfifo: u32,
        wfifo_wprt: u32,
        wfifo_rptr: u32,
        rfifo: u32,
        rfifo_wptr: u32,
        rfifo_rptr: u32,
        resv: [u32; 2],
        wfifo_status: u32,
        rfifo_status: u32,
    }
}

pub const I2C0_BASE: u32 = PERIPH_BASE + 0x7000;
pub const I2C1_BASE: u32 = PERIPH_BASE + 0x14000;
pub const I2C2_BASE: u32 = PERIPH_BASE + 0x15000;
pub static I2C0: I2c = I2c::new(I2C0_BASE);
pub static I2C1: I2c = I2c::new(I2C1_BASE);
pub static I2C2: I2c = I2c::new(I2C2_BASE);

pub const I2C_CR_RFIFO_OVERRUN_INTR_EN_MASK: u32 = 0x80000000;
pub const I2C_CR_RFIFO_FULL_INTR_EN_MASK: u32 = 0x40000000;
pub const I2C_CR_RFIFO_HALFFULL_INTR_EN_MASK: u32 = 0x20000000;
pub const I2C_CR_TFIFO_EMPTY_INTR_EN_MASK: u32 = 0x10000000;
pub const I2C_CR_TRANS_DONE_INTR_EN_MASK: u32 = 0x08000000;
pub const I2C_CR_MASTER_STOP_DET_EN_MASK: u32 = 0x04000000;
pub const I2C_CR_MASTER_STOP_DET_INTR_EN_MASK: u32 = 0x02000000;
pub const I2C_CR_SLAVE_STOP_DET_INTR_EN_MASK: u32 = 0x01000000;
pub const I2C_CR_SLAVE_ADDR_DET_INTR_EN_MASK: u32 = 0x00800000;
pub const I2C_CR_BUS_ERROR_INTR_EN_MASK: u32 = 0x00400000;
pub const I2C_CR_GENERAL_CALL_DIS_MASK: u32 = 0x00200000;
pub const I2C_CR_DBR_FULL_INTR_EN_MASK: u32 = 0x00100000;
pub const I2C_CR_IDBR_EMPTY_INTR_EN_MASK: u32 = 0x00080000;
pub const I2C_CR_ARB_LOSS_DET_INTR_EN_MASK: u32 = 0x00040000;

pub const I2C_CR_TWSI_UNIT_EN_MASK: u32 = 0x00004000;
pub const I2C_CR_SCL_EN_MASK: u32 = 0x00002000;
pub const I2C_CR_MASTER_ABORT_MASK: u32 = 0x00001000;
pub const I2C_CR_BUS_RESET_REQUEST_MASK: u32 = 0x00000800;
pub const I2C_CR_UNIT_RESET_MASK: u32 = 0x00000400;

pub const I2C_CR_BUS_MODE_MASK: u32 = 0x00000300;
pub const I2C_CR_BUS_MODE_STANDARD: u32 = 0x00000000;
pub const I2C_CR_BUS_MODE_FAST: u32 = 0x00000100;
pub const I2C_CR_BUS_MODE_HIGH: u32 = 0x00000200;

pub const I2C_CR_DMA_EN_MASK: u32 = 0x00000080;

pub const I2C_CR_FIFO_EN_MASK: u32 = 0x00000020;

pub const I2C_CR_TRANS_BEGIN_MASK: u32 = 0x00000010;
pub const I2C_CR_TRANS_BYTE_MASK: u32 = 0x00000008;
pub const I2C_CR_ACKNAK_MASK: u32 = 0x00000004;
pub const I2C_CR_STOP_MASK: u32 = 0x00000002;
pub const I2C_CR_START_MASK: u32 = 0x00000001;

/****************************I2C SR bit definition*****************************/
pub const I2C_SR_RFIFO_OVERRUN_MASK: u32 = 0x80000000;
pub const I2C_SR_RFIFO_FULL_MASK: u32 = 0x40000000;
pub const I2C_SR_RFIFO_HALFFULL_MASK: u32 = 0x20000000;
pub const I2C_SR_TFIFO_EMPTY_MASK: u32 = 0x10000000;
pub const I2C_SR_TRANS_DONE_MASK: u32 = 0x08000000;
pub const I2C_SR_MASTER_STOP_DET_MASK: u32 = 0x02000000;
pub const I2C_SR_SLAVE_STOP_DET_MASK: u32 = 0x01000000;
pub const I2C_SR_SLAVE_ADDR_DET_MASK: u32 = 0x00800000;
pub const I2C_SR_BUS_ERROR_MASK: u32 = 0x00400000;
pub const I2C_SR_GENERAL_CALL_MASK: u32 = 0x00200000;
pub const I2C_SR_DBR_FULL_MASK: u32 = 0x00100000;
pub const I2C_SR_IDBR_EMPTY_MASK: u32 = 0x00080000;
pub const I2C_SR_ARB_LOSS_DET_MASK: u32 = 0x00040000;

pub const I2C_SR_BUS_BUSY_MASK: u32 = 0x00010000;
pub const I2C_SR_UNIT_BUSY_MASK: u32 = 0x00008000;
pub const I2C_SR_ACK_STATUS_MASK: u32 = 0x00004000;
pub const I2C_SR_RW_MODE_MASK: u32 = 0x00002000;

/****************************I2C WFIFO bit definition************************/
pub const I2C_WFIFO_CONTROL_TB_MASK: u32 = 0x00000800;
pub const I2C_WFIFO_CONTROL_ACKNAK_MASK: u32 = 0x00000400;
pub const I2C_WFIFO_CONTROL_STOP_MASK: u32 = 0x00000200;
pub const I2C_WFIFO_CONTROL_START_MASK: u32 = 0x00000100;

/****************************I2C WFIFO_STATUS bit definition*****************/
pub const I2C_WFIFO_STATUS_SIZE_MASK: u32 = 0x0000003C;
pub const I2C_WFIFO_STATUS_EMPTY_MASK: u32 = 0x00000002;
pub const I2C_WFIFO_STATUS_FULL_MASK: u32 = 0x00000001;

/****************************I2C RFIFO_STATUS bit definition*****************/
pub const I2C_RFIFO_STATUS_SIZE_MASK: u32 = 0x000000F0;
pub const I2C_RFIFO_STATUS_EMPTY_MASK: u32 = 0x00000004;
pub const I2C_RFIFO_STATUS_FULL_MASK: u32 = 0x00000003;
pub const I2C_RFIFO_STATUS_HFULL_MASK: u32 = 0x00000002;
pub const I2C_RFIFO_STATUS_OVERUN_MASK: u32 = 0x00000001;

define_reg! {
    Syscfg
    __Syscfg {
        cr0: u32,
        cr1: u32,
        cr2: u32,
        cr3: u32,
        cr4: u32,
        cr5: u32,
        cr6: u32,
        cr7: u32,
        cr8: u32,
        cr9: u32,
        cr10: u32,
    }
}

pub const SYSCFG_BASE: u32 = PERIPH_BASE + 0x1000;
pub static SYSCFG: Syscfg = Syscfg::new(SYSCFG_BASE);

define_reg! {
    /// Wrapper over the raw PWR struct [`__Pwr`]
    Pwr
    /// Raw PWR struct
    __Pwr {
        /// control register 0, offset 0x00
        cr0: u32,
        /// control register 1, offset 0x04
        cr1: u32,
        /// status register 0, offset 0x08
        sr0: u32,
        /// status register 2, offset 0x0C
        sr1: u32,
        /// control register 3, offset 0x10
        cr2: u32,
        /// control register 4, offset 0x14
        cr3: u32,
        /// control register 5, offset 0x18
        cr4: u32,
        /// control register 6, offset 0x1C
        cr5: u32,
    }
}

pub const PWR_BASE: u32 = PERIPH_BASE + 0x1800;
pub static PWR: Pwr = Pwr::new(PWR_BASE);

define_reg! {
    TimerGp
    __TimerGp {
        /// TIMER control register 1, Address offset: 0x00
        cr1: u32,
        /// TIMER control register 2, Address offset: 0x04
        cr2: u32,
        /// TIMER slave Mode Control register, Address offset: 0x08
        smcr: u32,
        /// TIMER DMA/interrupt enable register, Address offset: 0x0C
        dier: u32,
        /// TIMER event generation register, Address offset: 0x14
        egr: u32,
        /// TIMER  capture/compare mode register 1, Address offset: 0x18
        ccmr1: u32,
        /// TIMER  capture/compare mode register 2, Address offset: 0x1C
        ccmr2: u32,
        /// TIMER capture/compare enable register, Address offset: 0x20
        ccer: u32,
        /// TIMER counter register, Address offset: 0x24
        cnt: u32,
        /// TIMER prescaler register, Address offset: 0x28
        psc: u32,
        /// TIMER auto-reload register, Address offset: 0x2C
        arr: u32,
        /// Reserved Address offset: 0x30
        resv1: u32,
        /// TIMER capture/compare register 0, Address offset: 0x34
        ccr0: u32,
        /// TIMER capture/compare register 1, Address offset: 0x38
        ccr1: u32,
        /// TIMER capture/compare register 2, Address offset: 0x3C
        ccr2: u32,
        /// TIMER capture/compare register 3, Address offset: 0x40
        ccr3: u32,
        /// Reserved, Address offset: 0x44
        resv2: u32,
        /// TIMER DMA control register, Address offset: 0x48
        dcr: u32,
        /// TIMER DMA address for full transfer register, Address offset: 0x4C
        dmar: u32,
        /// TIMER option register, Address offset: 0x50
        or: u32,
    }
}

pub const TIMER0_SFR_BASE: u32 = 0x4000A000;
pub const TIMER1_SFR_BASE: u32 = 0x4001A000;
pub const TIMER2_SFR_BASE: u32 = 0x4000B000;
pub const TIMER3_SFR_BASE: u32 = 0x4001B000;
pub static TIMER0: TimerGp = TimerGp::new(TIMER0_SFR_BASE);
pub static TIMER1: TimerGp = TimerGp::new(TIMER1_SFR_BASE);
pub static TIMER2: TimerGp = TimerGp::new(TIMER2_SFR_BASE);
pub static TIMER3: TimerGp = TimerGp::new(TIMER3_SFR_BASE);

define_reg! {
    Lptimer
    __Lptimer {
        /// LPTIMER flag and status register
        isr: u32,
        /// LPTIMER flag clear register
        icr: u32,
        /// LPTIMER interrupt enable register
        ier: u32,
        /// LPTIMER configuration register
        cfgr: u32,
        /// LPTIMER control register
        cr: u32,
        /// LPTIMER compare register
        cmp: u32,
        /// LPTIMER autoreload register
        arr: u32,
        /// LPTIMER counter register
        cnt: u32,
        /// LPTIMER CSR register
        csr: u32,
        /// LPTIMER SR1 register
        sr1: u32,
    }
}

pub const LPTIMER0_SFR_BASE: u32 = 0x4000D000;
pub const LPTIMER1_SFR_BASE: u32 = 0x4000D800;
pub static LPTIMER0: Lptimer = Lptimer::new(LPTIMER0_SFR_BASE);
pub static LPTIMER1: Lptimer = Lptimer::new(LPTIMER1_SFR_BASE);

define_reg! {
    I2s
    __I2s {
        /// enable register, offset 0x00
        ier: u32,
        /// receiver block enable register, offset 0x04
        irer: u32,
        /// transmitter block enable register, offset 0x08
        iter: u32,
        /// clock enable register, offset 0x0c
        cer: u32,
        /// clock configuration register, offset 0x10
        ccr: u32,
        /// receiver block FIFO reset register, offset
        rxffr: u32,
        /// transmitter block FIFO reset register, offset 0x18
        txffr: u32,
        /// reserved
        resv0: u32,

        /// right receive buffer register, offset 0x20
        lrbr_lthr: u32,
        /// right transmit holding register, offset 0x24
        rrbr_rthr: u32,
        /// receiver enable register, offset 0x28
        rer: u32,
        /// transmitter enable register, offset 0x2c
        ter: u32,
        /// receiver configuration register, offset 0x30
        rcr: u32,
        /// transmitter configuration register, offset 0x34
        tcr: u32,
        /// interrupt status register, offset 0x38
        isr: u32,
        /// interrupt mask register, offset 0x3c
        imr: u32,
        /// receiver overrun register, offset 0x40
        ror: u32,
        /// transmitter overrun register, offset 0x44
        tor: u32,
        /// receiver FIFO configuration register, offset 0x48
        rfcr: u32,
        /// transmitter FIFO configuration register, offset 0x4c
        tfcr: u32,
        /// receiver FIFO flush register, offset 0x50
        rff: u32,
        /// transmitter FIFO flush register, offset 0x54
        tff: u32,
        /// reserved
        resv1: [u32; 0x5a],
        /// receiver block dma register, offset 0x1c0
        rxdma: u32,
        /// reset receiver block dma register, offset 0x1c4
        rrxdma: u32,
        /// transmitter block dma register, offset 0x1c8
        txdma: u32,
        /// reset transmitter block dma register, offset 0x1cc
        rtxdma: u32,
        /// reserved
        resv2: [u32; 8],
        /// component parameter register 2, offset 0x1f0
        i2s_comp_param_2: u32,
        /// component parameter register 1, offset 0x1f4
        i2s_comp_param_1: u32,
        /// component version register, offset 0x1f8
        i2s_comp_version: u32,
        /// component type register, offset 0x1fc
        i2s_comp_type: u32,
    }
}

pub const I2S_BASE: u32 = 0x40002000;
pub static I2S: I2s = I2s::new(I2S_BASE);

define_reg! {
    Bstimer
    __Bstimer {
        cr1: u32,
        cr2: u32,
        resv1: u32,
        dier: u32,
        sr: u32,
        egr: u32,
        resv2: [u32; 3],
        cnt: u32,
        psc: u32,
        arr: u32,
    }
}

pub const BSTIMER0_SFR_BASE: u32 = 0x4000C000;
pub const BSTIMER1_SFR_BASE: u32 = 0x4001C000;
pub static BSTIMER0: Bstimer = Bstimer::new(BSTIMER0_SFR_BASE);
pub static BSTIMER1: Bstimer = Bstimer::new(BSTIMER1_SFR_BASE);

define_reg! {
    Sec
    __Sec {
        int: u32,
        rst: u32,
        sr: u32,
        filter0: u32,
        filter1: u32,
        filter2: u32,
        filter3: u32,
    }
}

pub const SEC_BASE: u32 = 0x4000F000;
pub static SEC: Sec = Sec::new(SEC_BASE);

pub const SEC_SR_FLASH_ACCESS_ERROR_MASK: u32 = 0x00001000;

define_reg! {
    Qspi
    __Qspi {
        qspi_cr: u32,
        qspi_dcr: u32,
        qspi_sr: u32,
        qspi_fcr: u32,
        qspi_dlr: u32,
        qspi_ccr: u32,
        qspi_ar: u32,
        qspi_abr: u32,
        qspi_dr: u32,
        qspi_psmkr: u32,
        qspi_psmar: u32,
        qspi_pir: u32,
        qspi_tor: u32,
        reserved: [u32; 19],
        qspi_hit0r: u32,
        qspi_hit1r: u32,
        qspi_mir: u32,
        qspi_cfgr: u32,
        sbus_start: u32,
    }
}

pub const QSPI_BASE: u32 = 0x40021000;
pub static QSPI: Qspi = Qspi::new(QSPI_BASE);

define_reg! {
    Dac
    __Dac {
        cr: u32,
        swtrigr: u32,
        dhr: u32,
        dor: u32,
        sr: u32,
    }
}

pub const DAC_BASE: u32 = 0x40019000;
pub static DAC: Dac = Dac::new(DAC_BASE);

pub const DAC_CR_INTR_EMPTY_EN_MASK: u32 = 0x00010000;
pub const DAC_CR_INTR_UNDERFLOW_EN_MASK: u32 = 0x00008000;

pub const DAC_CR_DMA_EN_MASK: u32 = 0x00004000;

pub const DAC_CR_MASK_AMP_SEL_MASK: u32 = 0x00003C00;
pub const DAC_CR_MASK_AMP_SEL_1: u32 = 0x00000000;
pub const DAC_CR_MASK_AMP_SEL_3: u32 = 0x00000400;
pub const DAC_CR_MASK_AMP_SEL_7: u32 = 0x00000800;
pub const DAC_CR_MASK_AMP_SEL_15: u32 = 0x00000C00;
pub const DAC_CR_MASK_AMP_SEL_31: u32 = 0x00001000;
pub const DAC_CR_MASK_AMP_SEL_63: u32 = 0x00001400;
pub const DAC_CR_MASK_AMP_SEL_127: u32 = 0x00001800;
pub const DAC_CR_MASK_AMP_SEL_255: u32 = 0x00001C00;
pub const DAC_CR_MASK_AMP_SEL_511: u32 = 0x00002000;
pub const DAC_CR_MASK_AMP_SEL_1023: u32 = 0x00002400;

pub const DAC_CR_WAVE_SEL_MASK: u32 = 0x00000300;
pub const DAC_CR_WAVE_SEL_NONE: u32 = 0x00000000;
pub const DAC_CR_WAVE_SEL_NOISE: u32 = 0x00000100;
pub const DAC_CR_WAVE_SEL_TRIANGLE: u32 = 0x00000200;

pub const DAC_CR_TRIG_TYPE_SEL_MASK: u32 = 0x000000C0;
pub const DAC_CR_TRIG_TYPE_SEL_RISING_EDGE: u32 = 0x00000000;
pub const DAC_CR_TRIG_TYPE_SEL_FALLING_EDGE: u32 = 0x00000040;
pub const DAC_CR_TRIG_TYPE_SEL_RISING_FALLING_EDGE: u32 = 0x00000080;

pub const DAC_CR_TRIG_SRC_SEL_MASK: u32 = 0x00000038;
pub const DAC_CR_TRIG_SRC_SEL_GPTIMER1_TRGO: u32 = 0x00000000;
pub const DAC_CR_TRIG_SRC_SEL_GPTIMER0_TRGO: u32 = 0x00000008;
pub const DAC_CR_TRIG_SRC_SEL_BSTIMER1_TRGO: u32 = 0x00000010;
pub const DAC_CR_TRIG_SRC_SEL_BSTIMER0_TRGO: u32 = 0x00000018;
pub const DAC_CR_TRIG_SRC_SEL_GPIO6: u32 = 0x00000020;
pub const DAC_CR_TRIG_SRC_SEL_GPIO24: u32 = 0x00000028;
pub const DAC_CR_TRIG_SRC_SEL_GPIO43: u32 = 0x00000030;
pub const DAC_CR_TRIG_SRC_SEL_SOFTWARE: u32 = 0x00000038;

pub const DAC_CR_TRIG_EN_MASK: u32 = 0x00000004;

pub const DAC_CR_DAC_EN_MASK: u32 = 0x00000001;

define_reg! {
    Adc
    __Adc {
        cr: u32,
        cfgr: u32,
        seqr0: u32,
        seqr1: u32,
        diffsel: u32,
        isr: u32,
        ier: u32,
        dr: u32,
        awd0_cfgr: u32,
        awd1_cfgr: u32,
        awd2_cfgr: u32,
    }
}

pub const ADC_BASE: u32 = 0x40017000;
pub static ADC: Adc = Adc::new(ADC_BASE);

define_reg! {
    Lcd
    __Lcd {
        cr0: u32,
        cr1: u32,
        dr0: u32,
        dr1: u32,
        dr2: u32,
        dr3: u32,
        dr4: u32,
        dr5: u32,
        dr6: u32,
        dr7: u32,
        sr: u32,
        cr2: u32,
    }
}

pub const LCD_BASE: u32 = 0x40018000;
pub static LCD: Lcd = Lcd::new(LCD_BASE);
