use crate::{
    ffi::{
        RCC_FREQ_4M, RCC_FREQ_24M, RCC_FREQ_32768, RCC_PCLK0, RCC_PCLK1, UART_CR_FLOW_CTRL,
        UART_CR_UART_EN, UART_CR_UART_MODE, UART_LCR_H_EPS_EVEN, UART_LCR_H_FEN, UART_LCR_H_PEN,
        UART_LCR_H_STOP, UART_LCR_H_WLEN, UART0_BASE, UART1_BASE, UART2_BASE, UART3_BASE,
    },
    regs::{RCC, tremo_reg_en, tremo_reg_set},
};

pub enum UartFlagStatus {
    Reset,
    Set,
}

pub struct UartConfig {
    pub baudrate: u32,
    pub data_width: DataWidth,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub flow_control: FlowControl,
    pub mode: Mode,
    pub fifo_mode: u8,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baudrate: 115200,
            data_width: DataWidth::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One,
            flow_control: FlowControl::Disabled,
            mode: Mode::TxRx,
            fifo_mode: 0,
        }
    }
}

#[repr(u32)]
pub enum UartFlag {
    TxFifoEmpty = 1 << 7,
    RxFifoFull = 1 << 6,
    TxFifoFull = 1 << 5,
    RxFifoEmpty = 1 << 4,
    Busy = 1 << 3,
}

#[repr(u32)]
pub enum Parity {
    None,
    Even,
    Odd,
}

#[repr(u32)]
pub enum DataWidth {
    Five = 0x00000000,
    Six = 0x00000020,
    Seven = 0x00000040,
    Eight = 0x00000060,
}

#[repr(u32)]
pub enum StopBits {
    One = 0x00000000,
    Two = 0x00000008,
}

#[repr(u32)]
pub enum FlowControl {
    Disabled = 0x00000000,
    Rts = 0x00004000,
    Cts = 0x00008000,
    CtsRts = 0x0000C000,
}

#[repr(u32)]
pub enum Mode {
    Tx = 0x00000010,
    Rx = 0x00000020,
    TxRx = 0x00000030,
}

/// wrapper over the raw UART struct [`__Uart`]
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

/// raw UART struct wrapper
pub struct Uart(*mut __Uart);

impl Uart {
    /// Create a new UART instance from base address
    pub const fn new(base: u32) -> Self {
        Self(base as *mut __Uart)
    }
    /// Initialize UART
    pub fn init(&mut self, config: UartConfig) -> Result<(), ()> {
        let uart = unsafe { &mut *self.0 };

        uart.cr &= !(UART_CR_UART_EN as u32); // disable UART
        uart.lcr_h &= !(UART_LCR_H_FEN as u32); // flush fifo
        uart.imsc = 0;

        let clk_src = unsafe {
            match self.0 as u32 {
                UART0_BASE => RCC.get_uart0_clk_source() >> 15,
                UART1_BASE => RCC.get_uart0_clk_source() >> 13,
                UART2_BASE => RCC.get_uart0_clk_source() >> 11,
                UART3_BASE => RCC.get_uart0_clk_source() >> 9,
                _ => 0,
            }
        };

        let uart_clk_freq = unsafe {
            match clk_src {
                1 => RCC_FREQ_4M,
                2 => RCC_FREQ_32768 as u32,
                3 => RCC_FREQ_24M,
                _ => {
                    if self.0 as u32 == UART0_BASE || self.0 as u32 == UART1_BASE {
                        RCC.get_clk_freq(RCC_PCLK0)
                    } else {
                        RCC.get_clk_freq(RCC_PCLK1)
                    }
                }
            }
        };

        if uart_clk_freq < 16 * config.baudrate {
            return Err(());
        }

        let br_div = calc_uart_baud(uart_clk_freq, config.baudrate);
        uart.ibrd = br_div >> 16;
        uart.fbrd = br_div & 0x3f;

        tremo_reg_set(
            &mut uart.lcr_h,
            UART_LCR_H_WLEN as u32,
            config.data_width as u32,
        );
        tremo_reg_set(
            &mut uart.lcr_h,
            UART_LCR_H_STOP as u32,
            config.stop_bits as u32,
        );
        tremo_reg_en(
            &mut uart.lcr_h,
            UART_LCR_H_FEN as u32,
            config.fifo_mode != 0,
        );

        match config.parity {
            Parity::Odd => {
                uart.lcr_h |= UART_LCR_H_PEN as u32;
                uart.lcr_h &= !(UART_LCR_H_EPS_EVEN as u32);
            }
            Parity::Even => {
                uart.lcr_h |= UART_LCR_H_PEN as u32;
                uart.lcr_h |= UART_LCR_H_EPS_EVEN as u32;
            }
            Parity::None => {
                uart.lcr_h &= !(UART_LCR_H_PEN as u32);
            }
        }

        tremo_reg_set(&mut uart.cr, UART_CR_UART_MODE as u32, config.mode as u32);
        tremo_reg_set(
            &mut uart.cr,
            UART_CR_FLOW_CTRL as u32,
            config.flow_control as u32,
        );

        Ok(())
    }
    /// Enable or disable the UART peripheral
    pub fn cmd(&mut self, new_state: bool) {
        let uart = unsafe { &mut *self.0 };
        tremo_reg_en(&mut uart.cr, UART_CR_UART_EN as u32, new_state);
    }
    /// Send a byte through UART
    pub fn send_data(&mut self, data: u8) {
        let uart = unsafe { &mut *self.0 };
        // wait till tx fifo is not full
        while matches!(
            self.get_flag_status(UartFlag::TxFifoFull),
            UartFlagStatus::Set
        ) {}
        uart.dr = data as u32;
    }
    /// Receive a byte through UART
    pub fn receive_data(&mut self) -> u8 {
        let uart = unsafe { &mut *self.0 };
        /* wait till rx fifo is not empty */
        while matches!(
            self.get_flag_status(UartFlag::RxFifoEmpty),
            UartFlagStatus::Set
        ) {}
        (uart.dr & 0xFF) as u8
    }
    /// Get UART flag status
    pub fn get_flag_status(&self, flag: UartFlag) -> UartFlagStatus {
        let uart = unsafe { &*self.0 };
        if (uart.fr & flag as u32) != 0 {
            UartFlagStatus::Set
        } else {
            UartFlagStatus::Reset
        }
    }
}

/// Calculate baud rate divisor
fn calc_uart_baud(uart_clk: u32, baud: u32) -> u32 {
    let mut temp = 16 * baud;
    if baud == 0 || uart_clk < temp {
        return 0;
    }

    let int_div = uart_clk / temp;
    let rem = uart_clk % temp;
    temp = 8 * rem / baud;
    let fac_div = (temp >> 1) | (temp & 1);
    (int_div << 16) | (fac_div & 0xFFFF)
}
