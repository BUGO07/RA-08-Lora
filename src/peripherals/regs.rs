use crate::cortex::{VolatileRO, VolatileRW};

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
    ($reg:expr, $mask:expr, $value:expr) => {
        $reg.write(($reg.read() & !($mask as u32)) | ($value as u32));
    };
}

/// Enable or disable bits in a register based on a mask
#[macro_export]
macro_rules! toggle_reg_bits {
    ($reg:expr, $mask:expr, $enable:expr) => {
        $reg.write(if $enable {
            $reg.read() | ($mask as u32)
        } else {
            $reg.read() & !($mask as u32)
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

        impl ::core::ops::Deref for $name {
            type Target = $raw_name;

            fn deref(&self) -> &Self::Target {
                unsafe { &*self.ptr }
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
        cr0: VolatileRW<u32>,
        cr1: VolatileRW<u32>,
        cr2: VolatileRW<u32>,
        cgr0: VolatileRW<u32>,
        cgr1: VolatileRW<u32>,
        cgr2: VolatileRW<u32>,
        rst0: VolatileRW<u32>,
        rst1: VolatileRW<u32>,
        rst_sr: VolatileRW<u32>,
        rst_cr: VolatileRW<u32>,
        sr: VolatileRO<u32>,
        sr1: VolatileRO<u32>,
        cr3: VolatileRW<u32>,
    }
}

#[allow(clippy::identity_op)]
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
        cr0: VolatileRW<u32>,
        cr1: VolatileRW<u32>,
        dr: VolatileRW<u32>,
        sr: VolatileRO<u32>,
        cpsr: VolatileRW<u32>,
        imsc: VolatileRW<u32>,
        ris: VolatileRO<u32>,
        mis: VolatileRO<u32>,
        icr: VolatileRW<u32>,
        dma_cr: VolatileRW<u32>,
        resv: [VolatileRO<u32>; 1006],
        periph_id0: VolatileRO<u32>,
        periph_id1: VolatileRO<u32>,
        periph_id2: VolatileRO<u32>,
        periph_id3: VolatileRO<u32>,
        pcell_id0: VolatileRO<u32>,
        pcell_id1: VolatileRO<u32>,
        pcell_id2: VolatileRO<u32>,
        pcell_id3: VolatileRO<u32>,
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
        ///  output enable register
        oer: VolatileRW<u32>,
        ///  output type register
        otyper: VolatileRW<u32>,
        ///  input enable register
        ier: VolatileRW<u32>,
        ///  pull enable register
        per: VolatileRW<u32>,
        ///  pull select register
        psr: VolatileRW<u32>,
        ///  input data register
        idr: VolatileRO<u32>,
        ///  output data register
        odr: VolatileRW<u32>,
        ///  bit reset register
        brr: VolatileRW<u32>,
        ///  bit set register
        bsr: VolatileRW<u32>,
        ///  dirve set register
        dsr: VolatileRW<u32>,
        ///  interrupt control register
        icr: VolatileRW<u32>,
        ///  interrupt flag register
        ifr: VolatileRW<u32>,
        ///  wakeup control register
        wucr: VolatileRW<u32>,
        ///  wakeup level register
        wulvl: VolatileRW<u32>,
        ///  alternate function low register
        afrl: VolatileRW<u32>,
        ///  alternate function high register
        afrh: VolatileRW<u32>,
        ///  stop3 wakeup control register
        stop3_wucr: VolatileRW<u32>,
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
        ctrl: VolatileRW<u32>,
        alarm0: VolatileRW<u32>,
        alarm1: VolatileRW<u32>,
        ppm_adjust: VolatileRW<u32>,
        calendar: VolatileRW<u32>,
        calendar_h: VolatileRW<u32>,
        cyc_max: VolatileRW<u32>,
        sr: VolatileRW<u32>,
        asyn_data: VolatileRO<u32>,
        asyn_data_h: VolatileRO<u32>,
        cr1: VolatileRW<u32>,
        sr1: VolatileRW<u32>,
        cr2: VolatileRW<u32>,
        sub_second_cnt: VolatileRO<u32>,
        cyc_cnt: VolatileRO<u32>,
        alarm0_subsecond: VolatileRW<u32>,
        alarm1_subsecond: VolatileRW<u32>,
        calendar_r: VolatileRW<u32>,
        calendar_r_h: VolatileRW<u32>,
    }
}

pub const RTC_REG_BASE: u32 = 0x4000E000;
pub static RTC: Rtc = Rtc::new(RTC_REG_BASE);

define_reg! {
    #[derive(Clone)]
    Uart
    __Uart {
        dr: VolatileRW<u32>,
        rsc_ecr: VolatileRW<u32>,
        rsv0: [VolatileRO<u32>; 4],
        fr: VolatileRO<u32>,
        rsv1: VolatileRO<u32>,
        ilpr: VolatileRW<u32>,
        ibrd: VolatileRW<u32>,
        fbrd: VolatileRW<u32>,
        lcr_h: VolatileRW<u32>,
        cr: VolatileRW<u32>,
        ifls: VolatileRW<u32>,
        imsc: VolatileRW<u32>,
        ris: VolatileRO<u32>,
        mis: VolatileRO<u32>,
        icr: VolatileRW<u32>,
        dmacr: VolatileRW<u32>,
        rsv2: [VolatileRO<u32>; 997],
        id: [VolatileRO<u32>; 8],
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
        cr0: VolatileRW<u32>,
        cr1: VolatileRW<u32>,
        sr0: VolatileRW<u32>,
        sr1: VolatileRW<u32>,
        data: VolatileRW<u32>,
    }
}

pub const LPUART_BASE: u32 = PERIPH_BASE + 0x5000;
pub static LPUART: Lpuart = Lpuart::new(LPUART_BASE);

define_reg! {
    Efc
    __Efc {
        cr: VolatileRW<u32>,
        int_en: VolatileRW<u32>,
        sr: VolatileRW<u32>,
        program_data0: VolatileRW<u32>,
        program_data1: VolatileRW<u32>,
        timing_cfg: VolatileRW<u32>,
        protect_seq: VolatileRW<u32>,
        rsv0: VolatileRW<u32>,
        chip_pattern: VolatileRO<u32>,
        ip_trim_l: VolatileRO<u32>,
        ip_trim_h: VolatileRO<u32>,
        sn_l: VolatileRO<u32>,
        sn_h: VolatileRO<u32>,
        test_info_l: VolatileRO<u32>,
        test_info_h: VolatileRO<u32>,
        option_csr_bytes: VolatileRO<u32>,
        option_e0_bytes: VolatileRO<u32>,
        option_wp_bytes: VolatileRO<u32>,
        option_sec_bytes0: VolatileRO<u32>,
        option_sec_bytes1: VolatileRO<u32>,
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
        ssp_cr0: VolatileRW<u32>,
        ssp_cr1: VolatileRW<u32>,
        ssp_dr: VolatileRW<u32>,
        ssp_sr: VolatileRO<u32>,
        ssp_cpsr: VolatileRW<u32>,
        ssp_imsc: VolatileRW<u32>,
        ssp_ris: VolatileRO<u32>,
        ssp_mis: VolatileRO<u32>,
        ssp_icr: VolatileRW<u32>,
        ssp_dma_cr: VolatileRW<u32>,
        rsv: [VolatileRO<u32>; 54],
        cr0: VolatileRW<u32>,
        cr1: VolatileRW<u32>,
        sr: VolatileRO<u32>,
        nss_cr: VolatileRW<u32>,
        sck_cr: VolatileRW<u32>,
        mosi_cr: VolatileRW<u32>,
        miso_sr: VolatileRW<u32>,
    }
}

pub const LORAC_BASE: u32 = PERIPH_BASE + 0x9000;
pub static LORAC: Lorac = Lorac::new(LORAC_BASE);
define_reg! {
    Afec
    __Afec {
        cr: VolatileRW<u32>,
        int_sr: VolatileRW<u32>,
        raw_sr: VolatileRO<u32>,
    }
}
pub static AFEC: Afec = Afec::new(AFEC_BASE + 0x200);

pub const AFEC_RAW_SR_RCO4M_READY_MASK: u32 = 0x80000000;

pub const AFEC_RAW_SR_PLL_UNLOCK_MASK: u32 = 0x40000000;

pub const AFEC_RAW_SR_RCO24M_READY_MASK: u32 = 0x00000004;
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
        load: VolatileRW<u32>,
        value: VolatileRO<u32>,
        control: VolatileRW<u32>,
        intclr: VolatileRW<u32>,
        ris: VolatileRO<u32>,
        mis: VolatileRO<u32>,
        dummy0: [VolatileRO<u32>; 0x2FA],
        lock: VolatileRW<u32>,
        dummy1: [VolatileRO<u32>; 0xBF],
        itcr: VolatileRW<u32>,
        itop: VolatileRW<u32>,
        dummy2: [VolatileRO<u32>; 0x32],
        periphid4: VolatileRO<u32>,
        periphid5: VolatileRO<u32>,
        periphid6: VolatileRO<u32>,
        periphid7: VolatileRO<u32>,
        periphid0: VolatileRO<u32>,
        periphid1: VolatileRO<u32>,
        periphid2: VolatileRO<u32>,
        periphid3: VolatileRO<u32>,
        pcellid0: VolatileRO<u32>,
        pcellid1: VolatileRO<u32>,
        pcellid2: VolatileRO<u32>,
        pcellid3: VolatileRO<u32>,
    }
}

pub const WDG_BASE: u32 = PERIPH_BASE + 0x1E000;
pub static WDG: Wdg = Wdg::new(WDG_BASE);

define_reg! {
    Crc
    __Crc {
        cr: VolatileRW<u32>,
        dr: VolatileRW<u32>,
        init: VolatileRW<u32>,
        poly: VolatileRW<u32>,
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
        cr: VolatileRW<u32>,
        sr: VolatileRW<u32>,
        sar: VolatileRW<u32>,
        dbr: VolatileRW<u32>,
        lcr: VolatileRW<u32>,
        wcr: VolatileRW<u32>,
        rst_cycl: VolatileRW<u32>,
        bmr: VolatileRO<u32>,
        wfifo: VolatileRW<u32>,
        wfifo_wprt: VolatileRW<u32>,
        wfifo_rptr: VolatileRW<u32>,
        rfifo: VolatileRW<u32>,
        rfifo_wptr: VolatileRW<u32>,
        rfifo_rptr: VolatileRW<u32>,
        resv: [VolatileRW<u32>; 2],
        wfifo_status: VolatileRO<u32>,
        rfifo_status: VolatileRO<u32>,
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
        cr0: VolatileRW<u32>,
        cr1: VolatileRW<u32>,
        cr2: VolatileRW<u32>,
        cr3: VolatileRW<u32>,
        cr4: VolatileRW<u32>,
        cr5: VolatileRW<u32>,
        cr6: VolatileRW<u32>,
        cr7: VolatileRW<u32>,
        cr8: VolatileRW<u32>,
        cr9: VolatileRW<u32>,
        cr10: VolatileRW<u32>,
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
        cr0: VolatileRW<u32>,
        /// control register 1, offset 0x04
        cr1: VolatileRW<u32>,
        /// status register 0, offset 0x08
        sr0: VolatileRW<u32>,
        /// status register 2, offset 0x0C
        sr1: VolatileRW<u32>,
        /// control register 3, offset 0x10
        cr2: VolatileRW<u32>,
        /// control register 4, offset 0x14
        cr3: VolatileRW<u32>,
        /// control register 5, offset 0x18
        cr4: VolatileRW<u32>,
        /// control register 6, offset 0x1C
        cr5: VolatileRW<u32>,
    }
}

pub const PWR_BASE: u32 = PERIPH_BASE + 0x1800;
pub static PWR: Pwr = Pwr::new(PWR_BASE);

define_reg! {
    TimerGp
    __TimerGp {
        /// TIMER control register 1, Address offset: 0x00
        cr1: VolatileRW<u32>,
        /// TIMER control register 2, Address offset: 0x04
        cr2: VolatileRW<u32>,
        /// TIMER slave Mode Control register, Address offset: 0x08
        smcr: VolatileRW<u32>,
        /// TIMER DMA/interrupt enable register, Address offset: 0x0C
        dier: VolatileRW<u32>,
        /// TIMER event generation register, Address offset: 0x14
        egr: VolatileRW<u32>,
        /// TIMER  capture/compare mode register 1, Address offset: 0x18
        ccmr1: VolatileRW<u32>,
        /// TIMER  capture/compare mode register 2, Address offset: 0x1C
        ccmr2: VolatileRW<u32>,
        /// TIMER capture/compare enable register, Address offset: 0x20
        ccer: VolatileRW<u32>,
        /// TIMER counter register, Address offset: 0x24
        cnt: VolatileRW<u32>,
        /// TIMER prescaler register, Address offset: 0x28
        psc: VolatileRW<u32>,
        /// TIMER auto-reload register, Address offset: 0x2C
        arr: VolatileRW<u32>,
        /// Reserved Address offset: 0x30
        resv1: VolatileRO<u32>,
        /// TIMER capture/compare register 0, Address offset: 0x34
        ccr0: VolatileRW<u32>,
        /// TIMER capture/compare register 1, Address offset: 0x38
        ccr1: VolatileRW<u32>,
        /// TIMER capture/compare register 2, Address offset: 0x3C
        ccr2: VolatileRW<u32>,
        /// TIMER capture/compare register 3, Address offset: 0x40
        ccr3: VolatileRW<u32>,
        /// Reserved, Address offset: 0x44
        resv2: VolatileRO<u32>,
        /// TIMER DMA control register, Address offset: 0x48
        dcr: VolatileRW<u32>,
        /// TIMER DMA address for full transfer register, Address offset: 0x4C
        dmar: VolatileRW<u32>,
        /// TIMER option register, Address offset: 0x50
        or: VolatileRW<u32>,
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

pub const LPTIMER0_SFR_BASE: u32 = 0x4000D000;
pub const LPTIMER1_SFR_BASE: u32 = 0x4000D800;
pub static LPTIMER0: Lptimer = Lptimer::new(LPTIMER0_SFR_BASE);
pub static LPTIMER1: Lptimer = Lptimer::new(LPTIMER1_SFR_BASE);

define_reg! {
    I2s
    __I2s {
        /// enable register, offset 0x00
        ier: VolatileRW<u32>,
        /// receiver block enable register, offset 0x04
        irer: VolatileRW<u32>,
        /// transmitter block enable register, offset 0x08
        iter: VolatileRW<u32>,
        /// clock enable register, offset 0x0c
        cer: VolatileRW<u32>,
        /// clock configuration register, offset 0x10
        ccr: VolatileRW<u32>,
        /// receiver block FIFO reset register, offset
        rxffr: VolatileRW<u32>,
        /// transmitter block FIFO reset register, offset 0x18
        txffr: VolatileRW<u32>,
        /// reserved
        resv0: VolatileRO<u32>,

        /// right receive buffer register, offset 0x20
        lrbr_lthr: VolatileRW<u32>,
        /// right transmit holding register, offset 0x24
        rrbr_rthr: VolatileRW<u32>,
        /// receiver enable register, offset 0x28
        rer: VolatileRW<u32>,
        /// transmitter enable register, offset 0x2c
        ter: VolatileRW<u32>,
        /// receiver configuration register, offset 0x30
        rcr: VolatileRW<u32>,
        /// transmitter configuration register, offset 0x34
        tcr: VolatileRW<u32>,
        /// interrupt status register, offset 0x38
        isr: VolatileRO<u32>,
        /// interrupt mask register, offset 0x3c
        imr: VolatileRW<u32>,
        /// receiver overrun register, offset 0x40
        ror: VolatileRO<u32>,
        /// transmitter overrun register, offset 0x44
        tor: VolatileRO<u32>,
        /// receiver FIFO configuration register, offset 0x48
        rfcr: VolatileRW<u32>,
        /// transmitter FIFO configuration register, offset 0x4c
        tfcr: VolatileRW<u32>,
        /// receiver FIFO flush register, offset 0x50
        rff: VolatileRW<u32>,
        /// transmitter FIFO flush register, offset 0x54
        tff: VolatileRW<u32>,
        /// reserved
        resv1: [VolatileRO<u32>; 0x5a],
        /// receiver block dma register, offset 0x1c0
        rxdma: VolatileRW<u32>,
        /// reset receiver block dma register, offset 0x1c4
        rrxdma: VolatileRW<u32>,
        /// transmitter block dma register, offset 0x1c8
        txdma: VolatileRW<u32>,
        /// reset transmitter block dma register, offset 0x1cc
        rtxdma: VolatileRW<u32>,
        /// reserved
        resv2: [VolatileRO<u32>; 8],
        /// component parameter register 2, offset 0x1f0
        i2s_comp_param_2: VolatileRO<u32>,
        /// component parameter register 1, offset 0x1f4
        i2s_comp_param_1: VolatileRO<u32>,
        /// component version register, offset 0x1f8
        i2s_comp_version: VolatileRO<u32>,
        /// component type register, offset 0x1fc
        i2s_comp_type: VolatileRO<u32>,
    }
}

pub const I2S_BASE: u32 = 0x40002000;
pub static I2S: I2s = I2s::new(I2S_BASE);

define_reg! {
    Bstimer
    __Bstimer {
        cr1: VolatileRW<u32>,
        cr2: VolatileRW<u32>,
        resv1: VolatileRO<u32>,
        dier: VolatileRW<u32>,
        sr: VolatileRO<u32>,
        egr: VolatileRW<u32>,
        resv2: [VolatileRO<u32>; 3],
        cnt: VolatileRW<u32>,
        psc: VolatileRW<u32>,
        arr: VolatileRW<u32>,
    }
}

pub const BSTIMER0_SFR_BASE: u32 = 0x4000C000;
pub const BSTIMER1_SFR_BASE: u32 = 0x4001C000;
pub static BSTIMER0: Bstimer = Bstimer::new(BSTIMER0_SFR_BASE);
pub static BSTIMER1: Bstimer = Bstimer::new(BSTIMER1_SFR_BASE);

define_reg! {
    Sec
    __Sec {
        int: VolatileRW<u32>,
        rst: VolatileRW<u32>,
        sr: VolatileRW<u32>,
        filter0: VolatileRW<u32>,
        filter1: VolatileRW<u32>,
        filter2: VolatileRW<u32>,
        filter3: VolatileRW<u32>,
    }
}

pub const SEC_BASE: u32 = 0x4000F000;
pub static SEC: Sec = Sec::new(SEC_BASE);

pub const SEC_SR_FLASH_ACCESS_ERROR_MASK: u32 = 0x00001000;

define_reg! {
    Qspi
    __Qspi {
        qspi_cr: VolatileRW<u32>,
        qspi_dcr: VolatileRW<u32>,
        qspi_sr: VolatileRW<u32>,
        qspi_fcr: VolatileRW<u32>,
        qspi_dlr: VolatileRW<u32>,
        qspi_ccr: VolatileRW<u32>,
        qspi_ar: VolatileRW<u32>,
        qspi_abr: VolatileRW<u32>,
        qspi_dr: VolatileRW<u32>,
        qspi_psmkr: VolatileRW<u32>,
        qspi_psmar: VolatileRW<u32>,
        qspi_pir: VolatileRW<u32>,
        qspi_tor: VolatileRW<u32>,
        reserved: [VolatileRW<u32>; 19],
        qspi_hit0r: VolatileRW<u32>,
        qspi_hit1r: VolatileRW<u32>,
        qspi_mir: VolatileRW<u32>,
        qspi_cfgr: VolatileRW<u32>,
        sbus_start: VolatileRW<u32>,
    }
}

pub const QSPI_BASE: u32 = 0x40021000;
pub static QSPI: Qspi = Qspi::new(QSPI_BASE);

define_reg! {
    Dac
    __Dac {
        cr: VolatileRW<u32>,
        swtrigr: VolatileRW<u32>,
        dhr: VolatileRW<u32>,
        dor: VolatileRO<u32>,
        sr: VolatileRW<u32>,
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
        cr: VolatileRW<u32>,
        cfgr: VolatileRW<u32>,
        seqr0: VolatileRW<u32>,
        seqr1: VolatileRW<u32>,
        diffsel: VolatileRW<u32>,
        isr: VolatileRW<u32>,
        ier: VolatileRW<u32>,
        dr: VolatileRO<u32>,
        awd0_cfgr: VolatileRW<u32>,
        awd1_cfgr: VolatileRW<u32>,
        awd2_cfgr: VolatileRW<u32>,
    }
}

pub const ADC_BASE: u32 = 0x40017000;
pub static ADC: Adc = Adc::new(ADC_BASE);

define_reg! {
    Lcd
    __Lcd {
        cr0: VolatileRW<u32>,
        cr1: VolatileRW<u32>,
        dr0: VolatileRW<u32>,
        dr1: VolatileRW<u32>,
        dr2: VolatileRW<u32>,
        dr3: VolatileRW<u32>,
        dr4: VolatileRW<u32>,
        dr5: VolatileRW<u32>,
        dr6: VolatileRW<u32>,
        dr7: VolatileRW<u32>,
        sr: VolatileRO<u32>,
        cr2: VolatileRW<u32>,
    }
}

pub const LCD_BASE: u32 = 0x40018000;
pub static LCD: Lcd = Lcd::new(LCD_BASE);
