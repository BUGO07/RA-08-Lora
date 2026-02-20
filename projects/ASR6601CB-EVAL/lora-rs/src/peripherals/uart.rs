use crate::{
    peripherals::{
        rcc::{
            RCC_FREQ_4M, RCC_FREQ_24M, RCC_FREQ_32768, RCC_PCLK0, RCC_PCLK1, RCC_PERIPHERAL_UART0,
            RCC_PERIPHERAL_UART1, RCC_PERIPHERAL_UART2, RCC_PERIPHERAL_UART3,
        },
        regs::*,
    },
    tremo_reg_en, tremo_reg_rd, tremo_reg_set, tremo_reg_wr,
};

/// UART configuration
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

/// UART flags
#[repr(u32)]
pub enum UartFlag {
    TxFifoEmpty = 1 << 7,
    RxFifoFull = 1 << 6,
    TxFifoFull = 1 << 5,
    RxFifoEmpty = 1 << 4,
    Busy = 1 << 3,
}

/// UART parity
#[repr(u32)]
pub enum Parity {
    None,
    Even,
    Odd,
}

/// UART data width
#[repr(u32)]
pub enum DataWidth {
    Five = 0x00000000,
    Six = 0x00000020,
    Seven = 0x00000040,
    Eight = 0x00000060,
}

/// UART stop bits
#[repr(u32)]
pub enum StopBits {
    One = 0x00000000,
    Two = 0x00000008,
}

/// UART flow control
#[repr(u32)]
pub enum FlowControl {
    Disabled = 0x00000000,
    Rts = 0x00004000,
    Cts = 0x00008000,
    CtsRts = 0x0000C000,
}

/// UART mode
#[repr(u32)]
pub enum Mode {
    Tx = 0x00000010,
    Rx = 0x00000020,
    TxRx = 0x00000030,
}

/// UART initialization error
#[derive(Debug)]
pub struct UartInitError;

impl Uart {
    /// Get UART flag status
    pub fn get_flag_status(&self, flag: UartFlag) -> SetStatus {
        let uart = unsafe { &*self.0 };
        if (uart.fr & flag as u32) != 0 {
            SetStatus::Set
        } else {
            SetStatus::Reset
        }
    }
    /// Send a byte through UART
    pub fn send_data(&mut self, data: u8) {
        let uart = unsafe { &mut *self.0 };
        // wait till tx fifo is not full
        while matches!(self.get_flag_status(UartFlag::TxFifoFull), SetStatus::Set) {}
        uart.dr = data as u32;
    }
    /// Receive a byte through UART
    pub fn receive_data(&mut self) -> u8 {
        let uart = unsafe { &mut *self.0 };
        /* wait till rx fifo is not empty */
        while matches!(self.get_flag_status(UartFlag::RxFifoEmpty), SetStatus::Set) {}
        (uart.dr & 0xFF) as u8
    }

    /// Config the interrupt of the specified UART flag
    pub fn config_interrupt(&mut self, uart_interrupt: u32, new_state: bool) {
        tremo_reg_en!(self, imsc, uart_interrupt, new_state);
    }

    /// Deinitializes the UART peripheral registers to the reset values
    pub fn deinit(&mut self) {
        let peripheral = match self.0 as u32 {
            UART0_BASE => RCC_PERIPHERAL_UART0,
            UART1_BASE => RCC_PERIPHERAL_UART1,
            UART2_BASE => RCC_PERIPHERAL_UART2,
            UART3_BASE => RCC_PERIPHERAL_UART3,
            _ => unreachable!(),
        };

        unsafe {
            RCC.enable_peripheral_clk(peripheral, false);
            RCC.rst_peripheral(peripheral, true);
            RCC.rst_peripheral(peripheral, false);
        }
    }

    /// Set the threshold of RX FIFO
    pub fn set_rx_fifo_threshold(&mut self, fifo_level: u32) {
        tremo_reg_set!(self, ifls, UART_IFLS_RX, fifo_level);
    }

    /// Set the threshold of TX FIFO
    pub fn set_tx_fifo_threshold(&mut self, fifo_level: u32) {
        tremo_reg_set!(self, ifls, UART_IFLS_TX, fifo_level);
    }

    /// Enable or disable the UART peripheral
    pub fn cmd(&mut self, new_state: bool) {
        tremo_reg_en!(self, cr, UART_CR_UART_EN, new_state);
    }

    /// Get the interrupt status of the UART interrupt
    pub fn get_interrupt_status(&self, interrupt: u32) -> SetStatus {
        if tremo_reg_rd!(self, mis) & interrupt != 0 {
            SetStatus::Set
        } else {
            SetStatus::Reset
        }
    }

    /// Get the interrupt status of the UART interrupt
    pub fn clear_interrupt(&mut self, interrupt: u32) {
        tremo_reg_wr!(self, icr, interrupt);
    }

    /// Initialize UART
    /// TODO: edit with macros
    pub fn init(&mut self, config: UartConfig) -> Result<(), UartInitError> {
        let uart = unsafe { &mut *self.0 };

        uart.cr &= !(UART_CR_UART_EN); // disable UART
        uart.lcr_h &= !(UART_LCR_H_FEN); // flush fifo
        uart.imsc = 0;

        let clk_src = unsafe {
            match self.0 as u32 {
                UART0_BASE => RCC.get_uart0_clk_src() >> 15,
                UART1_BASE => RCC.get_uart0_clk_src() >> 13,
                UART2_BASE => RCC.get_uart0_clk_src() >> 11,
                UART3_BASE => RCC.get_uart0_clk_src() >> 9,
                _ => 0,
            }
        };

        let uart_clk_freq = unsafe {
            match clk_src {
                1 => RCC_FREQ_4M,
                2 => RCC_FREQ_32768,
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
            return Err(UartInitError);
        }

        let br_div = calc_uart_baud(uart_clk_freq, config.baudrate);
        uart.ibrd = br_div >> 16;
        uart.fbrd = br_div & 0x3f;

        tremo_reg_set!(self, lcr_h, UART_LCR_H_WLEN, config.data_width);
        tremo_reg_set!(self, lcr_h, UART_LCR_H_STOP, config.stop_bits);
        tremo_reg_en!(self, lcr_h, UART_LCR_H_FEN, config.fifo_mode != 0);

        match config.parity {
            Parity::Odd => {
                uart.lcr_h |= UART_LCR_H_PEN;
                uart.lcr_h &= !(UART_LCR_H_EPS_EVEN);
            }
            Parity::Even => {
                uart.lcr_h |= UART_LCR_H_PEN;
                uart.lcr_h |= UART_LCR_H_EPS_EVEN;
            }
            Parity::None => {
                uart.lcr_h &= !(UART_LCR_H_PEN);
            }
        }

        tremo_reg_set!(self, cr, UART_CR_UART_MODE, config.mode);
        tremo_reg_set!(self, cr, UART_CR_FLOW_CTRL, config.flow_control);

        Ok(())
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
