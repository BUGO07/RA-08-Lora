use crate::ffi::{AFEC_BASE, GPIO_BASE, LORAC_BASE};

pub const PERIPH_BASE: u32 = 0x40000000;
pub const UART0_BASE: u32 = PERIPH_BASE + 0x3000;
pub const UART1_BASE: u32 = PERIPH_BASE + 0x4000;
pub const UART2_BASE: u32 = PERIPH_BASE + 0x10000;
pub const UART3_BASE: u32 = PERIPH_BASE + 0x11000;
pub const GPIOA_BASE: u32 = GPIO_BASE;
pub const GPIOB_BASE: u32 = GPIO_BASE + 0x400;
pub const GPIOC_BASE: u32 = GPIO_BASE + 0x800;
pub const GPIOD_BASE: u32 = GPIO_BASE + 0xC00;
pub const RCC_BASE: u32 = PERIPH_BASE;

pub static mut UART0: Uart = Uart::new(UART0_BASE);
pub static mut UART1: Uart = Uart::new(UART1_BASE);
pub static mut UART2: Uart = Uart::new(UART2_BASE);
pub static mut UART3: Uart = Uart::new(UART3_BASE);
pub static mut RCC: Rcc = Rcc::new(RCC_BASE);
pub static mut AFEC: Afec = Afec::new(AFEC_BASE + 0x200);
pub static mut LORAC: Lorac = Lorac::new(LORAC_BASE);
pub static mut GPIOA: Gpio = Gpio::new(GPIOA_BASE);
pub static mut GPIOB: Gpio = Gpio::new(GPIOB_BASE);
pub static mut GPIOC: Gpio = Gpio::new(GPIOC_BASE);
pub static mut GPIOD: Gpio = Gpio::new(GPIOD_BASE);

/// Uart Status
#[repr(u32)]
pub enum SetStatus {
    Reset = 0,
    Set = !0,
}

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

/// Read from a register
#[macro_export]
macro_rules! tremo_reg_rd {
    ($obj:expr, $reg:ident) => {
        unsafe { core::ptr::read_volatile(&(*$obj.0).$reg as *const u32) }
    };
}

/// Write to a register
#[macro_export]
macro_rules! tremo_reg_wr {
    ($obj:expr, $reg:ident, $value:expr) => {
        unsafe {
            core::ptr::write_volatile(&mut (*$obj.0).$reg, $value as u32);
        };
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

/// Read from analog register at address
#[macro_export]
macro_rules! tremo_analog_rd {
    ($addr:expr) => {{
        let addr = $addr;
        unsafe { core::ptr::read_volatile(($crate::ffi::AFEC_BASE | (addr << 2)) as *const u32) }
    }};
}

/// Write to analog register at address
#[macro_export]
macro_rules! tremo_analog_wr {
    ($addr:expr, $value:expr) => {
        let addr = $addr;
        let value = $value;
        unsafe {
            core::ptr::write_volatile(($crate::ffi::AFEC_BASE | (addr << 2)) as *mut u32, value)
        }
    };
}
