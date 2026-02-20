#![allow(clippy::identity_op)]

/// Uart Status
#[repr(u32)]
pub enum SetStatus {
    Reset = 0,
    Set = !0,
}

/// Read from a register
#[macro_export]
macro_rules! tremo_reg_rd {
    ($obj:expr, $reg:ident) => {
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*$obj.0).$reg)) }
    };
}

/// Write to a register
#[macro_export]
macro_rules! tremo_reg_wr {
    ($obj:expr, $reg:ident, $value:expr) => {
        unsafe {
            core::ptr::write_volatile(core::ptr::addr_of_mut!((*$obj.0).$reg), $value as u32);
        };
    };
}

/// Read from analog register at address
#[macro_export]
macro_rules! tremo_analog_rd {
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
macro_rules! tremo_analog_wr {
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
macro_rules! tremo_reg_set {
    ($obj:expr, $reg:ident, $mask:expr, $value:expr) => {
        unsafe {
            let ptr = &mut (*$obj.0).$reg;
            let current = core::ptr::read_volatile(ptr);
            let new_val = (current & !($mask as u32)) | ($value as u32);
            core::ptr::write_volatile(ptr, new_val);
        };
    };
}

/// Enable or disable bits in a register based on a mask
#[macro_export]
macro_rules! tremo_reg_en {
    ($obj:expr, $reg:ident, $mask:expr, $enable:expr) => {
        #[allow(clippy::macro_metavars_in_unsafe)]
        unsafe {
            let ptr = &mut (*$obj.0).$reg;
            let current = core::ptr::read_volatile(ptr);
            let new_val = if $enable {
                current | ($mask as u32)
            } else {
                current & !($mask as u32)
            };
            core::ptr::write_volatile(ptr, new_val);
        };
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

/// raw RCC struct
#[repr(C)]
pub struct __Rcc {
    pub cr0: u32,
    pub cr1: u32,
    pub cr2: u32,
    pub cgr0: u32,
    pub cgr1: u32,
    pub cgr2: u32,
    pub rst0: u32,
    pub rst1: u32,
    pub rst_sr: u32,
    pub rst_cr: u32,
    pub sr: u32,
    pub sr1: u32,
    pub cr3: u32,
}

/// wrapper over the raw RCC struct [`__Rcc`]
pub struct Rcc(pub *mut __Rcc);

impl Rcc {
    /// Create a new RCC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Rcc)
    }
}

pub const RCC_BASE: u32 = PERIPH_BASE + 0x00000000;
pub static mut RCC: Rcc = Rcc::new(RCC_BASE);

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

/// raw SSP struct
#[repr(C)]
pub struct __Ssp {
    pub cr0: u32,
    pub cr1: u32,
    pub dr: u32,
    pub sr: u32,
    pub cpsr: u32,
    pub imsc: u32,
    pub ris: u32,
    pub mis: u32,
    pub icr: u32,
    pub dma_cr: u32,
    pub resv: [u32; 1006],
    pub periph_id0: u32,
    pub periph_id1: u32,
    pub periph_id2: u32,
    pub periph_id3: u32,
    pub pcell_id0: u32,
    pub pcell_id1: u32,
    pub pcell_id2: u32,
    pub pcell_id3: u32,
}

/// wrapper over the raw SSP struct [`__Ssp`]
pub struct Ssp(pub *mut __Ssp);

impl Ssp {
    /// Create a new SSP instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Ssp)
    }
}

pub const SSP0_BASE: u32 = PERIPH_BASE + 0x00006000;
pub const SSP1_BASE: u32 = PERIPH_BASE + 0x00012000;
pub const SSP2_BASE: u32 = PERIPH_BASE + 0x00013000;
pub static mut SSP0: Ssp = Ssp::new(SSP0_BASE);
pub static mut SSP1: Ssp = Ssp::new(SSP1_BASE);
pub static mut SSP2: Ssp = Ssp::new(SSP2_BASE);

pub const SSP_NUM_PORTS: u32 = 3;

/// raw GPIO struct
#[repr(C)]
pub struct __Gpio {
    pub oer: u32,
    pub otyper: u32,
    pub ier: u32,
    pub per: u32,
    pub psr: u32,
    pub idr: u32,
    pub odr: u32,
    pub brr: u32,
    pub bsr: u32,
    pub dsr: u32,
    pub icr: u32,
    pub ifr: u32,
    pub wucr: u32,
    pub wulvl: u32,
    pub afrl: u32,
    pub afrh: u32,
    pub stop3_wucr: u32,
}

/// wrapper over the raw GPIO struct [`__Gpio`]
pub struct Gpio(pub *mut __Gpio);

impl Gpio {
    /// Create a new GPIO instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Gpio)
    }
}

pub const GPIO_BASE: u32 = 0x4001F000;
pub const GPIOA_BASE: u32 = GPIO_BASE;
pub const GPIOB_BASE: u32 = GPIO_BASE + 0x400;
pub const GPIOC_BASE: u32 = GPIO_BASE + 0x800;
pub const GPIOD_BASE: u32 = GPIO_BASE + 0xC00;
pub static mut GPIOA: Gpio = Gpio::new(GPIOA_BASE);
pub static mut GPIOB: Gpio = Gpio::new(GPIOB_BASE);
pub static mut GPIOC: Gpio = Gpio::new(GPIOC_BASE);
pub static mut GPIOD: Gpio = Gpio::new(GPIOD_BASE);

/// raw RTC struct
#[repr(C)]
pub struct __Rtc {
    pub ctrl: u32,
    pub alarm0: u32,
    pub alarm1: u32,
    pub ppm_adjust: u32,
    pub calendar: u32,
    pub calendar_h: u32,
    pub cyc_max: u32,
    pub sr: u32,
    pub asyn_data: u32,
    pub asyn_data_h: u32,
    pub cr1: u32,
    pub sr1: u32,
    pub cr2: u32,
    pub sub_second_cnt: u32,
    pub cyc_cnt: u32,
    pub alarm0_subsecond: u32,
    pub alarm1_subsecond: u32,
    pub calendar_r: u32,
    pub calendar_r_h: u32,
}

/// wrapper over the raw RTC struct [`__Rtc`]
pub struct Rtc(pub *mut __Rtc);

impl Rtc {
    /// Create a new RTC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Rtc)
    }
}

pub const RTC_REG_BASE: u32 = 0x4000E000;
pub static mut RTC: Rtc = Rtc::new(RTC_REG_BASE);

/// raw UART struct
#[repr(C)]
pub struct __Uart {
    pub dr: u32,
    pub rsc_ecr: u32,
    pub rsv0: [u32; 4],
    pub fr: u32,
    pub rsv1: u32,
    pub ilpr: u32,
    pub ibrd: u32,
    pub fbrd: u32,
    pub lcr_h: u32,
    pub cr: u32,
    pub ifls: u32,
    pub imsc: u32,
    pub ris: u32,
    pub mis: u32,
    pub icr: u32,
    pub dmacr: u32,
    pub rsv2: [u32; 997],
    pub id: [u32; 8],
}

/// wrapper over the raw UART struct [`__Uart`]
pub struct Uart(pub *mut __Uart);

impl Uart {
    /// Create a new UART instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Uart)
    }
}

pub const UART0_BASE: u32 = PERIPH_BASE + 0x3000;
pub const UART1_BASE: u32 = PERIPH_BASE + 0x4000;
pub const UART2_BASE: u32 = PERIPH_BASE + 0x10000;
pub const UART3_BASE: u32 = PERIPH_BASE + 0x11000;

pub static mut UART0: Uart = Uart::new(UART0_BASE);
pub static mut UART1: Uart = Uart::new(UART1_BASE);
pub static mut UART2: Uart = Uart::new(UART2_BASE);
pub static mut UART3: Uart = Uart::new(UART3_BASE);

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

/// raw LPUART struct
#[repr(C)]
pub struct __Lpuart {
    pub cr0: u32,
    pub cr1: u32,
    pub sr0: u32,
    pub sr1: u32,
    pub data: u32,
}

/// wrapper over the raw LPUART struct [`__Lpuart`]
pub struct Lpuart(pub *mut __Lpuart);

impl Lpuart {
    /// Create a new LPUART instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Lpuart)
    }
}

pub const LPUART_BASE: u32 = PERIPH_BASE + 0x5000;
pub static mut LPUART: Lpuart = Lpuart::new(LPUART_BASE);

/// raw EFC struct
#[repr(C)]
pub struct __Efc {
    pub cr: u32,
    pub int_en: u32,
    pub sr: u32,
    pub program_data0: u32,
    pub program_data1: u32,
    pub timing_cfg: u32,
    pub protect_seq: u32,
    pub rsv0: u32,
    pub chip_pattern: u32,
    pub ip_trim_l: u32,
    pub ip_trim_h: u32,
    pub sn_l: u32,
    pub sn_h: u32,
    pub test_info_l: u32,
    pub test_info_h: u32,
    pub option_csr_bytes: u32,
    pub option_e0_bytes: u32,
    pub option_wp_bytes: u32,
    pub option_sec_bytes0: u32,
    pub option_sec_bytes1: u32,
}

/// wrapper over the raw EFC struct [`__Efc`]
pub struct Efc(pub *mut __Efc);

impl Efc {
    /// Create a new EFC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Efc)
    }
}

pub const EFC_BASE: u32 = PERIPH_BASE + 0x20000;
pub static mut EFC: Efc = Efc::new(EFC_BASE);

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

/// raw LORAC struct
#[repr(C)]
pub struct __Lorac {
    pub ssp_cr0: u32,
    pub ssp_cr1: u32,
    pub ssp_dr: u32,
    pub ssp_sr: u32,
    pub ssp_cpsr: u32,
    pub ssp_imsc: u32,
    pub ssp_ris: u32,
    pub ssp_mis: u32,
    pub ssp_icr: u32,
    pub ssp_dma_cr: u32,
    pub rsv: [u32; 54],
    pub cr0: u32,
    pub cr1: u32,
    pub sr: u32,
    pub nss_cr: u32,
    pub sck_cr: u32,
    pub mosi_cr: u32,
    pub miso_sr: u32,
}

/// wrapper over the raw LORAC struct [`__Lorac`]
pub struct Lorac(pub *mut __Lorac);

impl Lorac {
    /// Create a new LORAC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Lorac)
    }
}

pub const LORAC_BASE: u32 = PERIPH_BASE + 0x9000;
pub static mut LORAC: Lorac = Lorac::new(LORAC_BASE);
/// raw AFEC struct
#[repr(C)]
pub struct __Afec {
    pub cr: u32,
    pub int_sr: u32,
    pub raw_sr: u32,
}

/// wrapper over the raw AFEC struct [`__Afec`]
pub struct Afec(pub *mut __Afec);

impl Afec {
    /// Create a new AFEC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Afec)
    }
}

pub static mut AFEC: Afec = Afec::new(AFEC_BASE + 0x200);

pub const AFEC_RAW_SR_RCO4M_READY_MASK: u32 = 0x80000000;

pub const AFEC_RAW_SR_PLL_UNLOCK_MASK: u32 = 0x40000000;

pub const AFEC_RAW_SR_RCO24M_READY_MASK: u32 = 0x00000004;
/// raw IWDG struct
#[repr(C)]
pub struct __Iwdg {
    pub cr: u32,
    pub max: u32,
    pub win: u32,
    pub sr: u32,
    pub sr1: u32,
    pub cr1: u32,
    pub sr2: u32,
}

/// wrapper over the raw IWDG struct [`__Iwdg`]
pub struct Iwdg(pub *mut __Iwdg);

impl Iwdg {
    /// Create a new IWDG instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Iwdg)
    }
}

pub const IWDG_BASE: u32 = PERIPH_BASE + 0x1D000;
pub static mut IWDG: Iwdg = Iwdg::new(IWDG_BASE);

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

/// raw WDG struct
#[repr(C)]
pub struct __Wdg {
    pub load: u32,
    pub value: u32,
    pub control: u32,
    pub intclr: u32,
    pub ris: u32,
    pub mis: u32,
    pub dummy0: [u32; 0x2FA],
    pub lock: u32,
    pub dummy1: [u32; 0xBF],
    pub itcr: u32,
    pub itop: u32,
    pub dummy2: [u32; 0x32],
    pub periphid4: u32,
    pub periphid5: u32,
    pub periphid6: u32,
    pub periphid7: u32,
    pub periphid0: u32,
    pub periphid1: u32,
    pub periphid2: u32,
    pub periphid3: u32,
    pub pcellid0: u32,
    pub pcellid1: u32,
    pub pcellid2: u32,
    pub pcellid3: u32,
}

/// wrapper over the raw WDG struct [`__Wdg`]
pub struct Wdg(pub *mut __Wdg);

impl Wdg {
    /// Create a new WDG instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Wdg)
    }
}

pub const WDG_BASE: u32 = PERIPH_BASE + 0x1E000;
pub static mut WDG: Wdg = Wdg::new(WDG_BASE);

/// raw CRC struct
#[repr(C)]
pub struct __Crc {
    pub cr: u32,
    pub dr: u32,
    pub init: u32,
    pub poly: u32,
}

/// wrapper over the raw CRC struct [`__Crc`]
pub struct Crc(pub *mut __Crc);

impl Crc {
    /// Create a new CRC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Crc)
    }
}

/// CRC base address
pub const CRC_BASE: u32 = PERIPH_BASE + 0x22000;
/// CRC peripheral
pub static mut CRC: Crc = Crc::new(CRC_BASE);

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
/// raw I2C struct
#[repr(C)]
pub struct __I2c {
    pub cr: u32,
    pub sr: u32,
    pub sar: u32,
    pub dbr: u32,
    pub lcr: u32,
    pub wcr: u32,
    pub rst_cycl: u32,
    pub bmr: u32,
    pub wfifo: u32,
    pub wfifo_wprt: u32,
    pub wfifo_rptr: u32,
    pub rfifo: u32,
    pub rfifo_wptr: u32,
    pub rfifo_rptr: u32,
    pub resv: [u32; 2],
    pub wfifo_status: u32,
    pub rfifo_status: u32,
}

/// wrapper over the raw I2C struct [`__I2c`]
pub struct I2c(pub *mut __I2c);

impl I2c {
    /// Create a new I2C instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __I2c)
    }
}

pub const I2C0_BASE: u32 = PERIPH_BASE + 0x7000;
pub const I2C1_BASE: u32 = PERIPH_BASE + 0x14000;
pub const I2C2_BASE: u32 = PERIPH_BASE + 0x15000;
pub static mut I2C0: I2c = I2c::new(I2C0_BASE);
pub static mut I2C1: I2c = I2c::new(I2C1_BASE);
pub static mut I2C2: I2c = I2c::new(I2C2_BASE);

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

/// raw SYSCFG struct
#[repr(C)]
pub struct __Syscfg {
    pub cr0: u32,
    pub cr1: u32,
    pub cr2: u32,
    pub cr3: u32,
    pub cr4: u32,
    pub cr5: u32,
    pub cr6: u32,
    pub cr7: u32,
    pub cr8: u32,
    pub cr9: u32,
    pub cr10: u32,
}

/// wrapper over the raw SYSCFG struct [`__Syscfg`]
pub struct Syscfg(pub *mut __Syscfg);

impl Syscfg {
    /// Create a new SYSCFG instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Syscfg)
    }
}

pub const SYSCFG_BASE: u32 = PERIPH_BASE + 0x1000;
pub static mut SYSCFG: Syscfg = Syscfg::new(SYSCFG_BASE);

/// raw PWR struct
#[repr(C)]
pub struct __Pwr {
    /// control register 0, offset 0x00
    pub cr0: u32,
    /// control register 1, offset 0x04
    pub cr1: u32,
    /// status register 0, offset 0x08
    pub sr0: u32,
    /// status register 2, offset 0x0C
    pub sr1: u32,
    /// control register 3, offset 0x10
    pub cr2: u32,
    /// control register 4, offset 0x14
    pub cr3: u32,
    /// control register 5, offset 0x18
    pub cr4: u32,
    /// control register 6, offset 0x1C
    pub cr5: u32,
}

/// wrapper over the raw PWR struct [`__Pwr`]
pub struct Pwr(pub *mut __Pwr);

impl Pwr {
    /// Create a new PWR instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Pwr)
    }
}

pub const PWR_BASE: u32 = PERIPH_BASE + 0x1800;
pub static mut PWR: Pwr = Pwr::new(PWR_BASE);

/// raw TIMER_GP struct
#[repr(C)]
pub struct __TimerGp {
    /// TIMER control register 1, Address offset: 0x00
    pub cr1: u32,
    /// TIMER control register 2, Address offset: 0x04
    pub cr2: u32,
    /// TIMER slave Mode Control register, Address offset: 0x08
    pub smcr: u32,
    /// TIMER DMA/interrupt enable register, Address offset: 0x0C
    pub dier: u32,
    /// TIMER event generation register, Address offset: 0x14
    pub egr: u32,
    /// TIMER  capture/compare mode register 1, Address offset: 0x18
    pub ccmr1: u32,
    /// TIMER  capture/compare mode register 2, Address offset: 0x1C
    pub ccmr2: u32,
    /// TIMER capture/compare enable register, Address offset: 0x20
    pub ccer: u32,
    /// TIMER counter register, Address offset: 0x24
    pub cnt: u32,
    /// TIMER prescaler register, Address offset: 0x28
    pub psc: u32,
    /// TIMER auto-reload register, Address offset: 0x2C
    pub arr: u32,
    /// Reserved Address offset: 0x30
    pub resv1: u32,
    /// TIMER capture/compare register 0, Address offset: 0x34
    pub ccr0: u32,
    /// TIMER capture/compare register 1, Address offset: 0x38
    pub ccr1: u32,
    /// TIMER capture/compare register 2, Address offset: 0x3C
    pub ccr2: u32,
    /// TIMER capture/compare register 3, Address offset: 0x40
    pub ccr3: u32,
    /// Reserved, Address offset: 0x44
    pub resv2: u32,
    /// TIMER DMA control register, Address offset: 0x48
    pub dcr: u32,
    /// TIMER DMA address for full transfer register, Address offset: 0x4C
    pub dmar: u32,
    /// TIMER option register, Address offset: 0x50
    pub or: u32,
}

/// wrapper over the raw TIMER_GP struct [`__TimerGp`]
pub struct TimerGp(pub *mut __TimerGp);

impl TimerGp {
    /// Create a new TIMER_GP instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __TimerGp)
    }
}

pub const TIMER0_SFR_BASE: u32 = 0x4000A000;
pub const TIMER1_SFR_BASE: u32 = 0x4001A000;
pub const TIMER2_SFR_BASE: u32 = 0x4000B000;
pub const TIMER3_SFR_BASE: u32 = 0x4001B000;
pub static mut TIMER0: TimerGp = TimerGp::new(TIMER0_SFR_BASE);
pub static mut TIMER1: TimerGp = TimerGp::new(TIMER1_SFR_BASE);
pub static mut TIMER2: TimerGp = TimerGp::new(TIMER2_SFR_BASE);
pub static mut TIMER3: TimerGp = TimerGp::new(TIMER3_SFR_BASE);

/// raw LPTIMER struct
#[repr(C)]
pub struct __Lptimer {
    /// LPTIMER flag and status register
    pub isr: u32,
    /// LPTIMER flag clear register
    pub icr: u32,
    /// LPTIMER interrupt enable register
    pub ier: u32,
    /// LPTIMER configuration register
    pub cfgr: u32,
    /// LPTIMER control register
    pub cr: u32,
    /// LPTIMER compare register
    pub cmp: u32,
    /// LPTIMER autoreload register
    pub arr: u32,
    /// LPTIMER counter register
    pub cnt: u32,
    /// LPTIMER CSR register
    pub csr: u32,
    /// LPTIMER SR1 register
    pub sr1: u32,
}

/// wrapper over the raw LPTIMER struct [`__Lptimer`]
pub struct Lptimer(pub *mut __Lptimer);

impl Lptimer {
    /// Create a new LPTIMER instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Lptimer)
    }
}

pub const LPTIMER0_SFR_BASE: u32 = 0x4000D000;
pub const LPTIMER1_SFR_BASE: u32 = 0x4000D800;
pub static mut LPTIMER0: Lptimer = Lptimer::new(LPTIMER0_SFR_BASE);
pub static mut LPTIMER1: Lptimer = Lptimer::new(LPTIMER1_SFR_BASE);

/// raw I2S struct
#[repr(C)]
pub struct __I2s {
    /// enable register, offset 0x00
    pub ier: u32,
    /// receiver block enable register, offset 0x04
    pub irer: u32,
    /// transmitter block enable register, offset 0x08
    pub iter: u32,
    /// clock enable register, offset 0x0c
    pub cer: u32,
    /// clock configuration register, offset 0x10
    pub ccr: u32,
    /// receiver block FIFO reset register, offset
    pub rxffr: u32,
    /// transmitter block FIFO reset register, offset 0x18
    pub txffr: u32,
    /// reserved
    pub resv0: u32,

    /// right receive buffer register, offset 0x20
    pub lrbr_lthr: u32,
    /// right transmit holding register, offset 0x24
    pub rrbr_rthr: u32,
    /// receiver enable register, offset 0x28
    pub rer: u32,
    /// transmitter enable register, offset 0x2c
    pub ter: u32,
    /// receiver configuration register, offset 0x30
    pub rcr: u32,
    /// transmitter configuration register, offset 0x34
    pub tcr: u32,
    /// interrupt status register, offset 0x38
    pub isr: u32,
    /// interrupt mask register, offset 0x3c
    pub imr: u32,
    /// receiver overrun register, offset 0x40
    pub ror: u32,
    /// transmitter overrun register, offset 0x44
    pub tor: u32,
    /// receiver FIFO configuration register, offset 0x48
    pub rfcr: u32,
    /// transmitter FIFO configuration register, offset 0x4c
    pub tfcr: u32,
    /// receiver FIFO flush register, offset 0x50
    pub rff: u32,
    /// transmitter FIFO flush register, offset 0x54
    pub tff: u32,
    /// reserved
    pub resv1: [u32; 0x5a],
    /// receiver block dma register, offset 0x1c0
    pub rxdma: u32,
    /// reset receiver block dma register, offset 0x1c4
    pub rrxdma: u32,
    /// transmitter block dma register, offset 0x1c8
    pub txdma: u32,
    /// reset transmitter block dma register, offset 0x1cc
    pub rtxdma: u32,
    /// reserved
    pub resv2: [u32; 8],
    /// component parameter register 2, offset 0x1f0
    pub i2s_comp_param_2: u32,
    /// component parameter register 1, offset 0x1f4
    pub i2s_comp_param_1: u32,
    /// component version register, offset 0x1f8
    pub i2s_comp_version: u32,
    /// component type register, offset 0x1fc
    pub i2s_comp_type: u32,
}

/// wrapper over the raw I2S struct [`__I2s`]
pub struct I2s(pub *mut __I2s);

impl I2s {
    /// Create a new I2S instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __I2s)
    }
}

pub const I2S_BASE: u32 = 0x40002000;
pub static mut I2S: I2s = I2s::new(I2S_BASE);

/// raw BSTIMER struct
#[repr(C)]
pub struct __Bstimer {
    pub cr1: u32,
    pub cr2: u32,
    pub resv1: u32,
    pub dier: u32,
    pub sr: u32,
    pub egr: u32,
    pub resv2: [u32; 3],
    pub cnt: u32,
    pub psc: u32,
    pub arr: u32,
}

/// wrapper over the raw BSTIMER struct [`__Bstimer`]
pub struct Bstimer(pub *mut __Bstimer);

impl Bstimer {
    /// Create a new BSTIMER instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Bstimer)
    }
}

pub const BSTIMER0_SFR_BASE: u32 = 0x4000C000;
pub const BSTIMER1_SFR_BASE: u32 = 0x4001C000;
pub static mut BSTIMER0: Bstimer = Bstimer::new(BSTIMER0_SFR_BASE);
pub static mut BSTIMER1: Bstimer = Bstimer::new(BSTIMER1_SFR_BASE);

/// raw SEC struct
#[repr(C)]
pub struct __Sec {
    pub int: u32,
    pub rst: u32,
    pub sr: u32,
    pub filter0: u32,
    pub filter1: u32,
    pub filter2: u32,
    pub filter3: u32,
}

/// wrapper over the raw SEC struct [`__Sec`]
pub struct Sec(pub *mut __Sec);

impl Sec {
    /// Create a new SEC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Sec)
    }
}

pub const SEC_BASE: u32 = 0x4000F000;
pub static mut SEC: Sec = Sec::new(SEC_BASE);

pub const SEC_SR_FLASH_ACCESS_ERROR_MASK: u32 = 0x00001000;

/// raw QSPI struct
#[repr(C)]
pub struct __Qspi {
    pub qspi_cr: u32,
    pub qspi_dcr: u32,
    pub qspi_sr: u32,
    pub qspi_fcr: u32,
    pub qspi_dlr: u32,
    pub qspi_ccr: u32,
    pub qspi_ar: u32,
    pub qspi_abr: u32,
    pub qspi_dr: u32,
    pub qspi_psmkr: u32,
    pub qspi_psmar: u32,
    pub qspi_pir: u32,
    pub qspi_tor: u32,
    pub reserved: [u32; 19],
    pub qspi_hit0r: u32,
    pub qspi_hit1r: u32,
    pub qspi_mir: u32,
    pub qspi_cfgr: u32,
    pub sbus_start: u32,
}

/// wrapper over the raw QSPI struct [`__Qspi`]
pub struct Qspi(pub *mut __Qspi);

impl Qspi {
    /// Create a new QSPI instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Qspi)
    }
}

pub const QSPI_BASE: u32 = 0x40021000;
pub static mut QSPI: Qspi = Qspi::new(QSPI_BASE);

/// raw DAC struct
#[repr(C)]
pub struct __Dac {
    pub cr: u32,
    pub swtrigr: u32,
    pub dhr: u32,
    pub dor: u32,
    pub sr: u32,
}

/// wrapper over the raw DAC struct [`__Dac`]
pub struct Dac(pub *mut __Dac);

impl Dac {
    /// Create a new DAC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Dac)
    }
}

pub const DAC_BASE: u32 = 0x40019000;
pub static mut DAC: Dac = Dac::new(DAC_BASE);

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

/// raw ADC struct
#[repr(C)]
pub struct __Adc {
    pub cr: u32,
    pub cfgr: u32,
    pub seqr0: u32,
    pub seqr1: u32,
    pub diffsel: u32,
    pub isr: u32,
    pub ier: u32,
    pub dr: u32,
    pub awd0_cfgr: u32,
    pub awd1_cfgr: u32,
    pub awd2_cfgr: u32,
}

/// wrapper over the raw ADC struct [`__Adc`]
pub struct Adc(pub *mut __Adc);

impl Adc {
    /// Create a new ADC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Adc)
    }
}

pub const ADC_BASE: u32 = 0x40017000;
pub static mut ADC: Adc = Adc::new(ADC_BASE);

/// raw LCD struct
#[repr(C)]
pub struct __Lcd {
    pub cr0: u32,
    pub cr1: u32,
    pub dr0: u32,
    pub dr1: u32,
    pub dr2: u32,
    pub dr3: u32,
    pub dr4: u32,
    pub dr5: u32,
    pub dr6: u32,
    pub dr7: u32,
    pub sr: u32,
    pub cr2: u32,
}

/// wrapper over the raw LCD struct [`__Lcd`]
pub struct Lcd(pub *mut __Lcd);

impl Lcd {
    /// Create a new LCD instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Lcd)
    }
}

pub const LCD_BASE: u32 = 0x40018000;
pub static mut LCD: Lcd = Lcd::new(LCD_BASE);
