use crate::ffi::{
    RCC_CR0_HCLK_DIV_MASK, RCC_CR0_PCLK0_DIV_MASK, RCC_CR0_PCLK1_DIV_MASK, RCC_CR0_SYSCLK_SEL_MASK,
    RCC_CR0_SYSCLK_SEL_RCO4M, RCC_CR0_SYSCLK_SEL_RCO32K, RCC_CR0_SYSCLK_SEL_RCO48M,
    RCC_CR0_SYSCLK_SEL_XO24M, RCC_CR0_SYSCLK_SEL_XO32K, RCC_CR0_SYSCLK_SEL_XO32M,
    RCC_CR2_UART0_CLK_SEL_MASK, RCC_FREQ_4M, RCC_FREQ_24M, RCC_FREQ_32M, RCC_FREQ_48M,
    RCC_FREQ_32000, RCC_FREQ_32768, RCC_HCLK, RCC_PCLK0, RCC_PCLK1,
};

/// raw RCC struct
#[repr(C)]
struct __Rcc {
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
pub struct Rcc(*mut __Rcc);

impl Rcc {
    /// Create a new RCC instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Rcc)
    }

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

    pub fn get_uart0_clk_source(&self) -> u32 {
        let rcc = unsafe { &*self.0 };
        rcc.cr2 & RCC_CR2_UART0_CLK_SEL_MASK as u32
    }
}
// fn rcc_get_clk_freq(clk: u32) -> u32 {
//     let mut tmp =
// }
