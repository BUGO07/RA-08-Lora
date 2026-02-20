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
